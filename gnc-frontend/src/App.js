import './App.css';
import {useEffect, useState} from "react";
import {Button, Card, CardLabelText, Footer, Header, TextArea, Title} from "./components/Basic"
import Logger from './utils/Logger'
import MonacoEditor from "@monaco-editor/react";
import AntVTree from "./components/AntVG6";
import DropMenu from "./components/DropMenu";
import Modal from "./components/Modal";
import {compile_result} from "gnc";
import AST2VisualizationData from "./utils/AST2VisData";
import evalAST from "./utils/HLVM";
import {ExampleOptions} from "./data/examples";
import useWindowSize from "./hook/useWindowSize";

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
    // visualization hooks
    const [parseTree, setParseTree] = useState({id: '0', label: 'GNC'})
    const [ast, setAST] = useState(null)
    const [visAST, setVisAST] = useState({id: '0', label: 'GNC'})
    const [visMode, setVisMode] = useState(VisOptions[0])

    // code hooks
    const [exampleCode, setExampleCode] = useState(ExampleOptions[0])
    const [code, editCode] = useState(exampleCode.code)

    // stdout hooks
    const [output, setOutput] = useState(`[INFO] Compile the code before running!`)

    // modal hooks
    const [isWarning, setIsWarning] = useState(false)
    const [showError, setShowError] = useState(false)
    const [errorTitle, setErrorTitle] = useState('')
    const [errorContent, setErrorContent] = useState('')

    // resize hooks
    const windowSize = useWindowSize()
    const [lastWindowSize, setLastWindowSize] = useState([0, 0])

    const handleExampleCodeChange = (option) => {
        setExampleCode(option)
        editCode(option.code)
    }

    const appendOutput = (data) => {
        setOutput(`${output}\n${data}`)
    }

    const showModal = (isError, title, content) => {
        setIsWarning(!isError)
        setErrorTitle(title)
        setErrorContent(content)
        setShowError(true)
    }

    const compile = () => {
        let data = JSON.parse(compile_result(code))
        if (!data.error) {
            setParseTree(data['parse_tree'])
            setAST(data['ast'])
            setVisAST(AST2VisualizationData(data['ast']))
            appendOutput(Logger.Info('Finished Compiling'))
        } else {
            setAST(null)
            appendOutput(Logger.Error('Compilation Error') + '\n' + data['error_message'])
            showModal(true, 'Compilation Error', data['error_message'])
        }
    }

    const run = () => {
        try {
            const res = evalAST(ast, 'main', [])
            // eslint-disable-next-line no-unused-vars
            const {type, value, stdout} = res
            appendOutput(Logger.Info('Finished Running') + '\n' + stdout + '\nExited with code ' + value)
        } catch (e) {
            appendOutput(Logger.Error('Runtime Error') + '\n' + e.message)
            showModal(true, 'Runtime Error', e.message)
        }
    }

    useEffect(() => {
        compile()
        // eslint-disable-next-line
    }, [])

    useEffect(() => {
        const [lastWidth, lastHeight] = lastWindowSize
        const [width, height] = windowSize
        if (width < lastWidth || height < lastHeight) {
            if (!showError) {
                showModal(false, 'Detected Window Shrink!', 'We detected a window shrink on your browser, please refresh to get better experience.')
            }
        }
        setLastWindowSize(windowSize)
        // eslint-disable-next-line
    }, [windowSize])

    return (
        <div className={'bg-green-100'}>
            <Modal
                isWarning={isWarning}
                visible={showError}
                title={errorTitle}
                content={errorContent}
                onClose={setShowError}
            />
            <Header>
                <div
                    className="text-4xl sm:text-6xl md:text-7xl lg:text-8xl text-center object-center align-middle font-extrabold w-full h-full">
                    <Title>
                        GNC Compiler Online
                    </Title>
                    <div className={'flex'}>
                        <div className={'flex flex-auto'}/>
                        <a className={'flex'} href={'https://github.com/PAN-Ziyue/GNC'}>
                            <img className={'flex mx-auto p-1'} src={'https://img.shields.io/github/stars/PAN-Ziyue/GNC'}
                                 alt={'GitHub Repo'}/>
                        </a>
                        <div className={'flex flex-auto'}/>
                    </div>
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
                        <MonacoEditor
                            defaultLanguage="c"
                            theme="light"
                            width={'100%'}
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
                            data={(visMode.id === 1) ? parseTree : visAST}
                        />
                    }
                />
                <Card
                    left={<CardLabelText>Running Result</CardLabelText>}
                    right={<Button onClick={run}>Run</Button>}
                    content={
                        <TextArea value={output}/>
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
