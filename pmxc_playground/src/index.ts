import * as monaco from "monaco-editor"

const THE_STATE: monaco.languages.IState = {
    clone: () => THE_STATE,
    equals: (_other: monaco.languages.IState) => true,
}

const getInitialState = (): monaco.languages.IState =>
    THE_STATE

const monacoTokenize = (doTokenize: (line: string) => any) => (line: string): monaco.languages.ILineTokens => {
    let result = doTokenize(line)
    console.log(result)
    return {
        tokens: result.tokens.map((token: any): monaco.languages.IToken => ({
            startIndex: token.startIndex as number,
            scopes: token.scopes as string,
        })),
        endState: THE_STATE,
    }
}

const monacoValidate = (editor: monaco.editor.ICodeEditor, doValidate: (sourceCode: string) => monaco.editor.IMarkerData[]) => {
    const model = editor.getModel()
    if (!model) {
        return
    }

    try {
        const sourceCode = model.getValue()
        const markers = doValidate(sourceCode) as monaco.editor.IMarkerData[]
        console.log(markers)

        monaco.editor.setModelMarkers(model, "pmxclang", markers)
    } catch (err) {
        console.error(err)
    }
}

type A = monaco.editor.IMarkerData

const main = async () => {
    const { tokenize: doTokenize, validate: doValidate } = await import("../dist/pmxc_analyzer")

    const editorElement = document.getElementById("editor")!

    monaco.languages.register({
        id: "pmxclang",
        aliases: [
            "pmxclang-lang",
        ],
        extensions: [
            ".pmxclang",
        ],
    })

    monaco.languages.setTokensProvider("pmxclang", {
        getInitialState,
        tokenize: monacoTokenize(doTokenize),
    })

    monaco.languages.setLanguageConfiguration("pmxclang", {
        comments: {
            lineComment: "//",
        },
        brackets: [
            ["(", ")"],
            ["[", "]"],
            ["{", "}"],
        ],
    })

    const editor = monaco.editor.create(editorElement, {
        value: `

enum Boolean {
    True,
    False,
}

// OK: 網羅的
match True {
    True => {}
    _ => {}
}

// OK: 網羅的
match True {
    True => {}
    False => {}
}

// NG: False が漏れる
match True {
    True => {}
    // False => {}
}
`,
        language: "pmxclang",
    })

    editor.onDidChangeModelContent(() => {
        monacoValidate(editor, doValidate)
    })

    monacoValidate(editor, doValidate)
}

document.addEventListener("DOMContentLoaded", main)
