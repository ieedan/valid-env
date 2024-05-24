const { Binary } = require("binary-install");
const os = require("os");

function getPlatform() {
	const type = os.type();
	const arch = os.arch();

	if (type === "Windows_NT") {
		if (arch === "x64") {
			return "win64";
		} else {
			return "win32";
		}
	}

	if (type === "Linux" && arch === "x64") {
		return "linux";
	}

	// Having issues getting this to compile so just skipping for now
	// if (type === "Darwin" && arch === "x64") {
	// 	return "macos";
	// }

	throw new Error(
		`Unsupported platform: ${type} ${arch}. Please create an issue at https://github.com/ieedan/vnv#issues`
	);
}

function getBinary() {
	const platform = getPlatform();
	const version = require("../package.json").version;
	const url = `https://github.com/ieedan/vnv/releases/download/v${version}/vnv-${platform}.tar.gz`;
	return new Binary(url, { name: "vnv" });
}

module.exports = getBinary;
