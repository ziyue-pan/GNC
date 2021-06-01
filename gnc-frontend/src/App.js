import './App.css';
import {useEffect, useState} from "react";
import {Button, Card, CardLabelText, Footer, Header, Input, Title} from "./components/Basic"
import Editor from "@monaco-editor/react";
import AntVTree from "./components/AntVG6";
import DropMenu from "./components/DropMenu";
import {compile_result} from "gnc";
import AST2VisualizationData from "./utils/AST2VisData";
import {ExampleOptions} from "./data/examples";

const VisOptions = [
    {
        id: 1,
        name: 'Parse Tree',
    },
    {
        id: 2,
        name: 'AST',
    },
]

function App() {
    // hooks
    const [parseTree, setParseTree] = useState({id: '0', label: 'GNC'})
    const [AST, setAST] = useState({id: '0', label: 'GNC'})
    const [visMode, setVisMode] = useState(VisOptions[0])
    const [exampleCode, setExampleCode] = useState(ExampleOptions[0])
    const [code, editCode] = useState(exampleCode.code)

    const compile = () => {
        let data = JSON.parse(compile_result(code))
        if (!data.error) {
            setParseTree(data['parse_tree'])
            setAST(AST2VisualizationData(data['ast']))
        } else {
            alert(data['error_message']) // TODO: Use Modal
        }
    }

    const handleExampleCodeChange = (option) => {
        setExampleCode(option)
        editCode(option.code)
    }

    useEffect(() => {
        compile()
        // eslint-disable-next-line
    }, [])

    return (
        <div className={'bg-green-100'}>
            <Header>
                <div
                    className="text-4xl sm:text-6xl md:text-7xl lg:text-8xl text-center object-center align-middle font-extrabold w-full h-full">
                    <Title>
                        GNC Compiler Online
                    </Title>
                    <a href={'https://github.com/PAN-Ziyue/GNC'}>
                        <img className={'mx-auto p-1'} src={'https://img.shields.io/github/stars/PAN-Ziyue/GNC'}
                             alt={'GitHub Repo'}/>
                    </a>
                </div>
            </Header>
            <div className={"flex flex-auto flex-col lg:flex-row"}>
                <Card
                    left={<CardLabelText>Code</CardLabelText>}
                    right={<DropMenu
                        widthClass={'w-44'}
                        options={ExampleOptions}
                        defaultOption={exampleCode}
                        onChange={handleExampleCodeChange}
                    />}
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
                        <div>
                            <CardLabelText>Visualization</CardLabelText>
                        </div>
                    }
                    right={
                        <div className={'flex flex-row'}>
                            <DropMenu
                                widthClass={'w-40'}
                                options={VisOptions}
                                defaultOption={visMode}
                                onChange={setVisMode}
                            />
                            <Button onClick={compile}>Compile</Button>
                        </div>
                    }
                    content={
                        <AntVTree
                            data={(visMode.id === 1) ? parseTree : AST}
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
                    <a href={'https://github.com/PAN-Ziyue/GNC'} className="no-underline hover:underline">GNC</a> (2021)
                    is the course project of Compilation Principle by Ziyue, MartinNose and <a
                    href={'https://www.ncj.wiki'} className="no-underline hover:underline">NCJ</a>. GNC is a recursive
                    acronym for "GNC's Not C-language!"
                </p>
            </Footer>
        </div>
    );
}

export default App;
