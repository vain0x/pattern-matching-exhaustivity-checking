import * as monaco from "monaco-editor"

const INITIAL_SOURCE_CODE = `enum Unit {
    Unit,
}

enum Boolean {
    False,
    True,
}

enum Digit {
    One,
    Two(Boolean),
}

// OK: 網羅的
match Unit {
    Unit => {}
}

// OK: 網羅的
match Unit {
    _ => {}
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

// OK: 網羅的
match One {
    One => {}
    Two(_) => {}
}

// OK: 網羅的
match One {
    One => {}
    Two(True) => {}
    Two(False) => {}
}

// OK: 網羅的 (冗長)
match True {
    True => {}
    False => {}
    True => {}
    False => {}
}

// NG: 非網羅的
match Unit {}

// NG: 非網羅的
match True {
    True => {}
    // False => {}
}

// NG: 非網羅的
match One {
    One => {}
    Two(True) => {}
    // Two(False) => {}
}

// その他

// NG: 型の異なるコンストラクタ
match Unit {
    True => {}
}

// NG: 存在しない型名
enum UsingNonExistingType {
    UsingNonExistingConstructor(NonExistingType),
}

// NG: 存在しないコンストラクタのパターン
match Unit {
    NonExistingConstructor => {}
}

// NG: 存在しないコンストラクタの式
match NonExistingConstructor {
    _ => {}
}

// NG: 引数の数が異なるパターン
match One {
    One(True) => {}
    Two => {}
    Two() => {}
    Two(True, True) => {}
}

// NG: 引数の型が異なるパターン
match One {
    Two(One) => {}
}
`

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
        automaticLayout: true,
        language: "pmxclang",
        value: INITIAL_SOURCE_CODE,
    })

    editor.onDidChangeModelContent(() => {
        monacoValidate(editor, doValidate)
    })

    monacoValidate(editor, doValidate)
}

document.addEventListener("DOMContentLoaded", main)
