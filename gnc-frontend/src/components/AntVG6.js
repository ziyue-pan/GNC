import React, {useEffect, useRef, useState} from 'react';
import ReactDOM from 'react-dom';
import G6 from '@antv/g6';

export default function AntVTree(props) {
    const style = {
        display: 'flex',
        flex: '1 1 auto'
    }
    const ref = useRef(null);
    const [graph, setGraph] = useState(null);
    // let graph = null;

    useEffect(() => {
        const {width, height} = ref.current.getBoundingClientRect()
        if (!graph) {
            const tooltip = new G6.Tooltip({
                offsetX: 10,
                offsetY: 20,
                getContent(e) {
                    const outDiv = document.createElement('div');
                    outDiv.style.width = '180px';
                    let content = `<h4>${e.item.getModel().label}</h4>`

                    if (e.item.getModel().attrs) {
                        const attrs = e.item.getModel().attrs
                        content += `<ul>`
                        for (const attr in attrs) {
                            content += `<li> - ${attr}: ${attrs[attr]}</li>`
                        }
                        content += `</ul>`
                    }
                    outDiv.innerHTML = content
                    return outDiv
                },
                itemTypes: ['node']
            });
            // eslint-disable-next-line
            let tmp_graph = new G6.TreeGraph({
                container: ReactDOM.findDOMNode(ref.current),
                width: width,
                height: height,
                linkCenter: true,
                modes: {
                    default: [
                        {
                            type: 'collapse-expand',
                            onChange: function onChange(item, collapsed) {
                                const data = item.getModel();
                                data.collapsed = collapsed;
                                return true;
                            },
                        },
                        'drag-canvas',
                        'zoom-canvas',
                    ],
                },
                defaultNode: {
                    size: 26,
                    anchorPoints: [
                        [0, 0.5],
                        [1, 0.5],
                    ],
                },
                defaultEdge: {
                    type: 'cubic-vertical',
                },
                layout: {
                    type: 'compactBox',
                    direction: 'TB',
                    getId: function getId(d) {
                        return d.id;
                    },
                    getHeight: function getHeight() {
                        return 16;
                    },
                    getWidth: function getWidth() {
                        return 16;
                    },
                    getVGap: function getVGap() {
                        return 80;
                    },
                    getHGap: function getHGap() {
                        return 20;
                    },
                },
                plugins: [tooltip],
            });
            tmp_graph.node(function (node) {
                let position = 'right';
                let rotate = 0;
                if (!node.children) {
                    position = 'bottom';
                    rotate = Math.PI / 2;
                }
                return {
                    style: {
                        fill: '#ECFDF5',
                        stroke: '#34D399',
                    },
                    label: node.label,
                    labelCfg: {
                        position,
                        offset: 5,
                        style: {
                            rotate,
                            textAlign: 'start',
                        },
                    },
                };
            });
            tmp_graph.data(props.data);
            tmp_graph.render();
            tmp_graph.fitView();
            setGraph(tmp_graph)
        } else {
            graph.data(props.data);
            graph.render();
            graph.fitView();
        }
    }, [graph, props.data]);

    return <div style={style} ref={ref}/>;
}