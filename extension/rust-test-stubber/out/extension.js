"use strict";
Object.defineProperty(exports, "__esModule", { value: true });
exports.deactivate = exports.activate = void 0;
const vscode = require("vscode");
const cp = require("child_process");
const fs = require("fs");
const tmp = require("tmp");
function activate(context) {
    let disposable = vscode.commands.registerCommand('rust-test-stubber.create-stubs', () => {
        let editor = vscode.window.activeTextEditor;
        if (!editor) {
            return;
        }
        let firstLine = editor.document.lineAt(0);
        let lastLine = editor.document.lineAt(editor.document.lineCount - 1);
        let textRange = new vscode.Range(firstLine.range.start, lastLine.range.end);
        let input = editor.document.getText(textRange);
        let inputFile = editor.document.uri.toString();
        tmp.file((err, path, _fd, cleanupCallback) => {
            if (err) {
                vscode.window.showErrorMessage(`I/O error: ${err}`);
                throw err;
            }
            cp.exec(`rust-test-stubber ${inputFile} ${path}`, (err, _stdin, _stdout) => {
                if (err) {
                    vscode.window.showErrorMessage(`I/O error: ${err}`);
                    throw err;
                }
                fs.readFile(path, 'utf8', (err, data) => {
                    if (err) {
                        vscode.window.showErrorMessage(`I/O error: ${err}`);
                        throw err;
                    }
                    editor === null || editor === void 0 ? void 0 : editor.edit(b => b.replace(textRange, data));
                    cleanupCallback();
                });
            });
        });
    });
    context.subscriptions.push(disposable);
}
exports.activate = activate;
function processString(input) {
    return input;
}
// this method is called when your extension is deactivated
function deactivate() { }
exports.deactivate = deactivate;
//# sourceMappingURL=extension.js.map