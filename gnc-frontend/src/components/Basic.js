import styled from "styled-components";

export const Button = styled.button.attrs({
    className: "flex flex-grow-0 py-2 px-4 ml-2 font-semibold rounded-lg shadow-md text-white bg-green-500 hover:bg-green-700"
})``;

export const Header = styled.header.attrs({
    className: "flex flex-auto"
})`
    height: 17vh;
`;

export const Footer = styled.footer.attrs({
    className: "flex flex-auto "
})`
    height: 3vh;
    min-height: 35px;
`;

export const CardLabelText = styled.span.attrs({
    className: "flex text-white text-3xl object-center font-bold pl-2 pt-1"
})``;

export const Title = styled.span.attrs({
    className: "bg-clip-text text-transparent bg-gradient-to-r from-green-400 to-blue-500"
})`
    line-height: 15vh;
`;

export const Input = styled.input.attrs({
    className: "text-green-900 rounded-lg shadow-md border-2 border-green-500"
})``;

export const TextArea = styled.textarea.attrs({
    disabled: true,
    className: "flex w-full h-full resize-none"
})``;

const CardContainer = styled.div.attrs({
    className: "flex flex-auto container w-full lg:w-1/3 p-2"
})`
    height: 80vh;
`;

const CardItem = styled.div.attrs({
    className: "flex flex-auto w-full h-full bg-white rounded-xl shadow-md overflow-hidden"
})``;

export function Card(props) {
    return (
        <CardContainer>
            <CardItem>
                <div className={'flex flex-auto flex-col h-full'}>
                    <div className={'flex flex-grow-0 flex-row p-2 bg-green-600'}>
                        {props.left}
                        <div className={'flex flex-grow'}/>
                        {props.right}
                    </div>
                    <div className={'flex flex-auto flex-row p-2'}>
                        {props.content}
                    </div>
                </div>
            </CardItem>
        </CardContainer>
    )
}