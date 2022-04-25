import path from 'path';
import {fileURLToPath} from 'url';

import {
    refactor
} from "shift-refactor";

import {
    readFileSync,
} from "fs";

const __filename = fileURLToPath(import.meta.url);
const __dirname = path.dirname(__filename);

const file=
    readFileSync(__dirname+"/../scripts/css_test.js","utf8")
    ||`
    console.log(1,2,3,4,5);
    const a='some string';
`;

const script=refactor(file);

console.log(file);

const start=performance.now()*1000; // micros

const arr=script.$(`Script LiteralNumericExpression + LiteralNumericExpression`);

const duration=Math.round(performance.now()*1000-start);

console.log(arr.nodes);
console.log("Matches:",arr.length);
console.log("Micros:",duration);
