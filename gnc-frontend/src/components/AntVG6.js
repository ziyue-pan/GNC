import React, {useEffect} from 'react';
import ReactDOM from 'react-dom';
import G6 from '@antv/g6';
import data from './test.json';

export default function AntVTree(props) {
    const ref = React.useRef(null);
    let graph = null;

    useEffect(() => {
        console.log(data)
        if (!graph) {
            // eslint-disable-next-line
            graph = new G6.TreeGraph({
                container: ReactDOM.findDOMNode(ref.current),
                width: 550,
                height: 700,
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
            });
        }
        graph.node(function (node) {
            let position = 'right';
            let rotate = 0;
            if (!node.children) {
                position = 'bottom';
                rotate = Math.PI / 2;
            }
            return {
                label: node.id,
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
        graph.data(data);
        graph.render();
        graph.fitView();
    }, []);

    return <div ref={ref}/>;
}