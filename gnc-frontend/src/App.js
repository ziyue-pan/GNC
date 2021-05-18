import './App.css';
import MonacoEditor from "react-monaco-editor";
import {useState} from "react";

function App() {
    const [code, editCode] = useState('')
    const options = {
        selectOnLineNumbers: true
    }
    const editorDidMount = (editor, monaco) => {
        console.log('editorDidMount', editor);
        editor.focus();
    }
    return (
        <MonacoEditor
            width="800"
            height="600"
            language="cpp"
            theme="vs-dark"
            value={code}
            options={options}
            onChange={editCode}
            editorDidMount={editorDidMount}
        />
    );
}

export default App;
