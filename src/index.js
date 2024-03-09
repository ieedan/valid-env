import fs from "fs-extra"

/** Parses the .env file with the path (path)
 * 
 * @param {string} path 
 * @param {import('.').ParseOptions} options 
 */
export default function parse(path, options){
    if (!fs.existsSync(path)) console.error(`Couldn't find .env file located at: ${path}`); 
    const content = fs.readFileSync(path, 'utf8');

    let i = 0;
    const currentDecorators = [];
    const keys = new Map();
    let isDecorator = false;

    while (i < content.length) {
        const char = content[i];
        if (char == "@") {
            isDecorator = true;
        }

        i++;
    }
}