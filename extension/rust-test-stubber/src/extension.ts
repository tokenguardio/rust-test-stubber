import * as vscode from 'vscode';

import cp = require('child_process');
import fs = require('fs');
import tmp = require('tmp');
import { assert } from 'console';

export function activate(context: vscode.ExtensionContext) {
	let createStubs = vscode.commands.registerCommand('stubber.create', () => {
		let editor = vscode.window.activeTextEditor;
		if (editor === undefined) {
			console.log('undefined editor');
			return;
		}
		let firstLine = editor.document.lineAt(0);
		let lastLine = editor.document.lineAt(editor.document.lineCount - 1);
		let textRange = new vscode.Range(firstLine.range.start, lastLine.range.end);
		let inputFile = editor.document.uri.fsPath;
		
		tmp.file((err, path, _fd, cleanupCallback) => {
			if (err) {
				vscode.window.showErrorMessage(`I/O error: ${err}`);
				return;
			}
			console.log(`test-stub-gen ${inputFile} ${path}`);
			cp.exec(`test-stub-gen ${inputFile} ${path}`, (err, _stdin, _stdout) => {
				if (err) {
					vscode.window.showErrorMessage(`I/O error: ${err}`);
					return;
				} 
				cp.exec(`rustfmt ${path}`, (err, _stdin, _stdout) => {
					if (err) {
						vscode.window.showErrorMessage(`I/O error: ${err}`);
						return;
					}
					fs.readFile(path, 'utf8', (err, data) => {
						if (err) {
							vscode.window.showErrorMessage(`I/O error: ${err}`);
							return;
						}
						editor?.edit(b => b.replace(textRange, data));
						cleanupCallback();
					});
				});
			});
		});
	});

	context.subscriptions.push(createStubs);
}

// this method is called when your extension is deactivated
export function deactivate() {}
