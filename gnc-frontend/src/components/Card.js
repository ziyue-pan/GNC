import styled from "styled-components";

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