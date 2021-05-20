import './App.css';
import MonacoEditor from "react-monaco-editor";
import {useState} from "react";
import styled from 'styled-components'

const Button = styled.button.attrs({
    className: "py-2 px-4 font-semibold rounded-lg shadow-md text-white bg-green-500 hover:bg-green-700"
})``;

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
        <div>
            <MonacoEditor
                width="800"
                height="600"
                language="cpp"
                theme="vs-light"
                value={code}
                options={options}
                onChange={editCode}
                editorDidMount={editorDidMount}
            />
            <Button>Normal Button</Button>
        </div>
    );
}

export default App;
