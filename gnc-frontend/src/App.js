import './App.css';
import {useState} from "react";
import styled from 'styled-components'
import Editor from "@monaco-editor/react";

const Button = styled.button.attrs({
    className: "py-2 px-4 font-semibold rounded-lg shadow-md text-white bg-green-500 hover:bg-green-700"
})``;

const Card = styled.div.attrs({
    className: "w-1/3 h-full mx-auto bg-white rounded-xl shadow-md space-y-2 sm:py-4 sm:flex sm:items-center sm:space-y-0 sm:space-x-6"
})`
height: 300px
`;

function App() {
    const [code, editCode] = useState('')
    return (
        <div style={{'height': '100%'}}>
            <Card>
                <Editor
                    // height="90vh"
                    defaultLanguage="c"
                    defaultValue="// some comment"
                    onChange={editCode}
                    value={code}
                />
            </Card>
            <Button>Normal Button</Button>
        </div>
    );
}

export default App;
