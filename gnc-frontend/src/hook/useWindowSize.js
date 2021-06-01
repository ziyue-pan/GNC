import {useLayoutEffect, useState} from "react";

function debounce(fn, ms) {
    let timer
    return _ => {
        clearTimeout(timer)
        timer = setTimeout(_ => {
            timer = null
            fn.apply(this, arguments)
        }, ms)
    };
}

export default function useWindowSize() {
    const [size, setSize] = useState([0, 0]);
    useLayoutEffect(() => {
        function updateSize() {
            setSize([window.innerWidth, window.innerHeight]);
        }
        const debouncedUpdateSize = debounce(updateSize, 1000)
        window.addEventListener('resize', debouncedUpdateSize);
        return () => window.removeEventListener('resize', updateSize);
    }, []);
    return size;
}
