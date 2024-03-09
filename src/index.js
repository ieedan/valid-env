import fs from "fs-extra";
import path from "node:path";

const DECORATORS = {
	private: () => true,
	public: () => true,
	max: (value, constraint, type) => {
		if (type == "string") {
			if (value.length > constraint) return false;
		} else {
			if (parseFloat(value) > constraint) return false;
		}

		return true;
	},
	min: (value, constraint, type) => {
		if (type == "string") {
			if (value.length < constraint) return false;
		} else {
			if (parseFloat(value) < constraint) return false;
		}

		return true;
	},
};

/**
 *
 * @param {string} val
 */
function removeQuotes(val) {
	if (val[0] == '"' && val[val.length - 1] == '"') {
		return val.slice(1, val.length - 1);
	} else if (val[0] == "'" && val[val.length - 1] == "'") {
		return val.slice(1, val.length - 1);
	} else {
		return val;
	}
}

/**
 *
 * @param {string} decorator
 */
function isolateDecorator(decorator) {
	const start = decorator.indexOf("(");
	if (start == -1) return { decorator, value: "" };

	const end = decorator.indexOf(")", start);
	const value = decorator.slice(start + 1, end);

	return { decorator: decorator.slice(0, start), value };
}

/** Values
 *
 * @param {Map<string, { value: string, scope: "public" | "private" }>} keyMap
 */
function writeToOutput(keyMap, outputDir) {
	outputDir = path.resolve(outputDir, "env");
	if (!fs.pathExistsSync(outputDir)) {
		fs.mkdirSync(outputDir, { recursive: true });
		fs.createFileSync(path.resolve(outputDir, "index.js"));
		fs.writeFileSync(
			path.resolve(outputDir, "index.js"),
			`import priv from "./private.js";
import pub from "./public.js";

/** @type {import('.').env} */
const env = {
	private: priv,
	public: pub
}

export default env;`
		);
		fs.createFileSync(path.resolve(outputDir, "private.js"));
		fs.createFileSync(path.resolve(outputDir, "public.js"));
		fs.createFileSync(path.resolve(outputDir, "index.d.ts"));
	}

	const priv = [];
	const pub = [];

	for (const [key, value] of keyMap) {
		if (value.scope == "private") {
			priv.push({ key, value: value.value });
		} else {
			pub.push({ key, value: value.value });
		}
	}

	fs.writeFileSync(
		path.resolve(outputDir, "public.js"),
		`const pub = {
	${pub.map((a) => {
		const isNum = isFinite(a.value);
		return `${a.key}: ${isNum ? a.value.toString() : `"${a.value}"`},\n`;
	})}
};

export default pub;`
	);

	fs.writeFileSync(
		path.resolve(outputDir, "private.js"),
		`const priv = {
	${priv.map((a) => {
		const isNum = isFinite(a.value);
		return `${a.key}: ${isNum ? a.value.toString() : `"${a.value}"`}`;
	}).join(",\n\t")}
};

export default priv;`
	);

	fs.writeFileSync(path.resolve(outputDir, "index.d.ts"), `export type env = {
	private: Private;
	public: Public;
};

export type Public = {
	${pub.map((a) => {
		const isNum = isFinite(a.value);
		return `${a.key}: ${isNum ? "number" : "string"}`;
	}).join(",\n\t")}
}

export type Private = {
	${priv.map((a) => {
		const isNum = isFinite(a.value);
		return `${a.key}: ${isNum ? "number" : "string"}`;
	}).join(",\n\t")}
}`)
}

/** Parses the .env file with the path (path)
 *
 * @param {string} path
 * @param {import('.').ParseOptions} options
 */
export default function parse(path, options) {
	if (!fs.existsSync(path)) {
		console.warn(`Couldn't find .env file located at: ${path}`);
		return;
	}
	const content = fs.readFileSync(path, "utf8");

	let i = 0;
	let currentDecorators = [];
	const keys = new Map();
	let isDecorator = false;
	let isValue = false;
	let isComment = false;
	let current = "";
	let currentKey = "";

	while (i < content.length) {
		const char = content[i];
		if (char == "@" && !isValue) {
			isDecorator = true;
		} else if (char == "#" && !isValue && !isDecorator) {
			isComment = true;
		} else if (char == " " || char == "\n" || char == "\r" || i == content.length - 1) {
			if (isDecorator) {
				currentDecorators.push(current);
				isDecorator = false;
			} else if (isValue) {
				if (i == content.length - 1) current += char;
				current = removeQuotes(current);
				if (keys.has(currentKey))
					console.warn(
						`Duplicate key ${currentKey} present in ${path} overwriting value to latest occurrence.`
					);

				const isNum = isFinite(current);

				let scope = "private";
				for (let d = 0; d < currentDecorators.length; d++) {
					const dec = isolateDecorator(currentDecorators[d]);
					if (DECORATORS[dec.decorator] == undefined) {
						throw new Error(`Invalid decorator ${dec.decorator}`);
					}

					if (dec.decorator == "private" || dec.decorator == "public") {
						scope = dec.decorator;
						continue;
					}

					// now validators

					const type = isNum ? "number" : "string";

					if (!DECORATORS[dec.decorator](current, dec.value, type)) {
						throw new Error(
							`Invalid variable value for ${currentKey} value must be ${dec.decorator} ${dec.value}`
						);
					}
				}

				if (isNum) {
					keys.set(currentKey, { value: parseFloat(current), scope });
				} else {
					keys.set(currentKey, { value: current, scope });
				}

				currentDecorators = [];
				currentKey = "";
				isValue = false;
			} else if (isComment) {
				isComment = false;
			} 

			current = "";
		} else if (char === "=" && !isValue && !isDecorator) {
			currentKey = current;
			isValue = true;
			current = "";
		} else {
			current += char;
		}

		i++;
	}

	console.log(keys)

	writeToOutput(keys, options.outputFile);
}
