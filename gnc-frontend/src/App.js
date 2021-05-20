import './App.css';
import {useState} from "react";
import styled from 'styled-components'
import Editor from "@monaco-editor/react";

const Button = styled.button.attrs({
    className: "flex flex-grow-0 py-2 px-4 font-semibold rounded-lg shadow-md text-white bg-green-500 hover:bg-green-700"
})``;

const CardContainer = styled.div.attrs({
    className: "flex flex-auto container w-1/3 p-2"
})`
    height: 80vh
`;

const Card = styled.div.attrs({
    className: "flex flex-auto w-full h-full bg-white rounded-xl shadow-md p-2"
})``;

const Header = styled.header.attrs({
    className: "flex flex-auto"
})`
    height: 15vh
`;

const Footer = styled.footer.attrs({
    className: "flex flex-auto"
})`
    height: 5vh
`;

function App() {
    const [code, editCode] = useState('#include <stdio.h>')
    return (
        <div className={'bg-green-100'}>
            <Header>
                <div className="text-4xl sm:text-6xl md:text-7xl lg:text-8xl text-center font-extrabold w-full pt-10 sm:pt-5">
                    <span className="bg-clip-text text-transparent bg-gradient-to-r from-green-400 to-blue-500">
                        GNC Compiler Online
                    </span>
                </div>
            </Header>
            <div className={"flex flex-auto flex-row"}>
                <CardContainer>
                    <Card>
                        <div className={'flex flex-auto flex-col h-full'}>
                            <div className={'flex flex-auto pb-2'}>
                                <Button>Run</Button>
                            </div>
                            <div className={'flex flex-auto'}>
                                <Editor
                                    defaultLanguage="c"
                                    onChange={editCode}
                                    value={code}
                                />
                            </div>
                        </div>
                    </Card>
                </CardContainer>
                <CardContainer>
                    <Card>
                        <Button>Compile</Button>
                    </Card>
                </CardContainer>
                <CardContainer>
                    <Card>
                        <Button>Run</Button>
                    </Card>
                </CardContainer>
            </div>
            <Footer>
                <p className="text-green-700 text-opacity-50 text-center w-full">
                    <a href={'https://github.com/PAN-Ziyue/GNC'}>
                        <img className={'mx-auto flex-grow-0'} src={'https://img.shields.io/github/stars/PAN-Ziyue/GNC'} alt={'GitHub Repo'}/>
                    </a>
                    <a href={'https://github.com/PAN-Ziyue/GNC'} className="no-underline hover:underline">GNC</a> (2021) is the course project of Compilation Principle by Ziyue, MartinNose and <a href={'https://www.ncj.wiki'} className="no-underline hover:underline">NCJ</a>. GNC is a recursive acronym for "GNC's Not C-language!"
                </p>
            </Footer>
        </div>
    );
}

export default App;
