import './App.css';
import {useState} from "react";
import styled from 'styled-components'
import Editor from "@monaco-editor/react";
import AntVTree from "./components/AntVG6";
import mockASTData from './components/test.json';
import {Card} from "./components/Card";
import {compile_result} from "gnc"
import AST2VisualizationData from "./utils/AST2VisData"

const Button = styled.button.attrs({
    className: "flex flex-grow-0 py-2 px-4 font-semibold rounded-lg shadow-md text-white bg-green-500 hover:bg-green-700"
})``;

const Header = styled.header.attrs({
    className: "flex flex-auto"
})`
    height: 17vh;
`;

const Footer = styled.footer.attrs({
    className: "flex flex-auto "
})`
    height: 3vh;
    min-height: 35px;
`;

const CardLabelText = styled.span.attrs({
    className: "flex text-white text-3xl object-center font-bold pl-2 pt-1"
})``;

const Title = styled.span.attrs({
    className: "bg-clip-text text-transparent bg-gradient-to-r from-green-400 to-blue-500"
})`
    line-height: 15vh;
`;

const Input = styled.input.attrs({
    className: "text-green-900 rounded-lg shadow-md border-2 border-green-500"
})``;

const InitialCode = `int main (int a) {
    return a;
}`

function App() {
    const [code, editCode] = useState(InitialCode)
    const [parseTree, setParseTree] = useState(mockASTData)

    const compile = () => {
        let data = JSON.parse(compile_result(code))
        if (!data.error) {
            setParseTree(AST2VisualizationData(data['ast']))
            // console.log(data['ast'])
        } else {
            alert(data['error_message']) // TODO: Use Modal
        }
    }

    return (
        <div className={'bg-green-100'}>
            <Header>
                <div className="text-4xl sm:text-6xl md:text-7xl lg:text-8xl text-center object-center align-middle font-extrabold w-full h-full">
                    <Title>
                        GNC Compiler Online
                    </Title>
                    <a href={'https://github.com/PAN-Ziyue/GNC'}>
                        <img className={'mx-auto flex-grow-0 p-1'} src={'https://img.shields.io/github/stars/PAN-Ziyue/GNC'} alt={'GitHub Repo'}/>
                    </a>
                </div>
            </Header>
            <div className={"flex flex-auto flex-col lg:flex-row"}>
                <Card
                    left={<CardLabelText>Code</CardLabelText>}
                    right={<Button>Run</Button>}
                    content={
                        <Editor
                            defaultLanguage="c"
                            theme="light"
                            onChange={editCode}
                            value={code}
                        />
                    }
                />
                <Card
                    left={
                        <div className={'flex flex-row'}>
                            <CardLabelText>Parse Tree</CardLabelText>
                            <CardLabelText>/</CardLabelText>
                            <CardLabelText className={'text-opacity-50'}>AST</CardLabelText>
                        </div>
                    }
                    right={<Button onClick={compile}>Compile</Button>}
                    content={
                        <AntVTree
                            data={parseTree}
                        />
                    }
                />
                <Card
                    left={<CardLabelText>Running Result</CardLabelText>}
                    right={<Button>Run</Button>}
                    content={
                        <div>
                            function: `main()`
                            <br/>
                            a: <Input/>
                            <br/>
                            res: 23333
                        </div>
                    }
                />
            </div>
            <Footer>
                <p className="text-xs lg:text-lg text-green-700 text-opacity-50 text-center object-center w-full">
                    <a href={'https://github.com/PAN-Ziyue/GNC'} className="no-underline hover:underline">GNC</a> (2021) is the course project of Compilation Principle by Ziyue, MartinNose and <a href={'https://www.ncj.wiki'} className="no-underline hover:underline">NCJ</a>. GNC is a recursive acronym for "GNC's Not C-language!"
                </p>
            </Footer>
        </div>
    );
}

export default App;
