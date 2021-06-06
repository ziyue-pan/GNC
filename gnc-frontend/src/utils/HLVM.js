import {vsprintf} from 'sprintf-js'
// const vsprintf = require('sprintf-js').vsprintf;

/***
 * HLVM - High Level Virtual Machine
 * @param ast AST to be interpreted
 * @param entryPoint entry function name
 * @param args arguments as AST, e.g. {"IntLiteral": 233}
 * @returns {type, value} result
 */
export default function evalAST(ast, entryPoint, args) {
    // the entering & leaving of scopes will be maintained by the visitor function
    let symbolTable = [{}]
    let callStack = []
    let stdout = ""

    const findSymbol = (symbolName) => {
        // iterate from top of the stack to emulate scope's name hiding
        for (const scopeTable of symbolTable.slice().reverse()) {
            if (symbolName in scopeTable) {
                return scopeTable[symbolName] // return ref of the symbol
            }
        }
        throw Error(`Can not find symbol ${symbolName}`)
    }

    const castValue = (origData, destType) => {
        console.log(destType)
        console.log(origData)
        let res = {
            type: destType,
            value: origData.value
        }
        switch (destType) {
            case "Int":
            case "Char":
            case "Short":
            case "Long":
                res.value = Math.floor(origData.value)
                break
            case "Bool":
                if (origData.type !== "Bool") {
                    res.value = origData.value !== 0
                }
                break
            case "Float":
            case "Double":
                res.value = origData.value
                break
            default:
                if (destType.Pointer === "Char") {
                    res.value = origData.value
                    res.type = "String"
                } else {
                    throw new Error(`Cast to ${JSON.stringify(destType)} not implemented`)
                }
        }
        return res
    }

    const visitNode = (astNode) => {
        let res = null
        for (const node of astNode) {
            // [Control Flow] if function has returned, stop execution
            if (callStack.length && callStack.slice(-1)[0].hasReturn) {
                return res
            }
            if (node === "BreakStatement" || node === "ContinueStatement") { // special statements
                switch (node) {
                    case "BreakStatement":
                        const loopStack = callStack.slice(-1)[0].loopStack
                        loopStack[loopStack.length - 1] = true
                        return res // [Control Flow] stop exec following blocks after break
                    case "ContinueStatement":
                        return res // [Control Flow] stop current loop and jump to next // FIXME seems buggy, but I don't want to think
                    default:
                        throw new Error('strange thing')
                }
            } else for (const x in node) { // Actually there's only one entry inside
                switch (x) {
                    // basics
                    case "Assignment":
                        const [assignOperation, leftValue, assignChildren] = node[x]
                        const variable = visitNode([leftValue]) // TODO implement real leftValue
                        res = castValue(visitNode([assignChildren]), variable.type)
                        switch (assignOperation) {
                            case "Simple":
                                variable.value = res.value
                                break
                            case "Addition":
                                variable.value += res.value
                                break
                            case "Subtraction":
                                variable.value -= res.value
                                break
                            case "Multiplication":
                                variable.value *= res.value
                                break
                            case "Division":
                                variable.value /= res.value
                                break
                            case "Modulus":
                                variable.value %= res.value
                                break
                            case "BitwiseAnd":
                                variable.value &= res.value
                                break
                            case "InclusiveOr":
                                variable.value |= res.value
                                break
                            case "ExclusiveOr":
                                variable.value ^= res.value
                                break
                            case "ShiftLeft":
                                variable.value <<= res.value
                                break
                            case "ShiftRight":
                                variable.value >>= res.value
                                break
                            default:
                                throw new Error(`${assignOperation} not implemented`)
                        }
                        res = variable
                        break
                    case "Identifier":
                        res = findSymbol(node[x])
                        break

                    // literals
                    case "IntLiteral":
                        res = {
                            type: "Int",
                            value: node[x]
                        }
                        break
                    case "BoolLiteral":
                        res = {
                            type: "Bool",
                            value: node[x]
                        }
                        break
                    case "FloatLiteral":
                        res = {
                            type: "Float",
                            value: node[x]
                        }
                        break
                    case "StringLiteral":
                        res = {
                            type: "String",
                            value: node[x]
                        }
                        break

                    // declarations
                    case "Declaration":
                        const [type, name] = node[x]
                        if (name in symbolTable.slice().reverse()[0]) {
                            throw new Error(`${name} already declared`)
                        }
                        symbolTable.slice(-1)[0][name] = {
                            type,
                            value: 0
                        }
                        break
                    case "GlobalDeclaration":
                        const [globalType, globalName, globalAssignment] = node[x]
                        symbolTable.slice(-1)[0][globalName] = {
                            type: globalType,
                            value: castValue(visitNode([globalAssignment]).value, globalType)
                        }
                        break

                    // expressions
                    case "BinaryExpression":
                        const [binaryOperator, lhs, rhs] = node[x]
                        const leftRes = visitNode([lhs])
                        const rightRes = visitNode([rhs])
                        switch (binaryOperator) {
                            case "Add":
                                res = {
                                    type: leftRes.type, // TODO handle cast
                                    value: leftRes.value + rightRes.value
                                }
                                break
                            case "Subtract":
                                res = {
                                    type: leftRes.type, // TODO handle cast
                                    value: leftRes.value - rightRes.value
                                }
                                break
                            case "Multiply":
                                res = {
                                    type: leftRes.type, // TODO handle cast
                                    value: leftRes.value * rightRes.value
                                }
                                break
                            case "Modulus":
                                res = {
                                    type: leftRes.type, // TODO handle cast
                                    value: leftRes.value % rightRes.value
                                }
                                break
                            case "Divide":
                                res = {
                                    type: leftRes.type, // TODO handle cast
                                    value: leftRes.value / rightRes.value
                                }
                                break
                            case "Equal":
                                res = {
                                    type: 'Bool',
                                    value: leftRes.value === rightRes.value
                                }
                                break
                            case "ShiftRight":
                                res = {
                                    type: leftRes.type, // TODO handle cast
                                    value: leftRes.value >> rightRes.value
                                }
                                break
                            case "ShiftLeft":
                                res = {
                                    type: leftRes.type, // TODO handle cast
                                    value: leftRes.value << rightRes.value
                                }
                                break
                            case "LessThan":
                                res = {
                                    type: 'Bool',
                                    value: leftRes.value < rightRes.value
                                }
                                break
                            case "GreaterThan":
                                res = {
                                    type: 'Bool',
                                    value: leftRes.value > rightRes.value
                                }
                                break
                            case "LessEqual":
                                res = {
                                    type: 'Bool',
                                    value: leftRes.value <= rightRes.value
                                }
                                break
                            case "GreaterEqual":
                                res = {
                                    type: 'Bool',
                                    value: leftRes.value >= rightRes.value
                                }
                                break
                            case "NotEqual":
                                res = {
                                    type: 'Bool',
                                    value: leftRes.value !== rightRes.value
                                }
                                break
                            case "BitwiseAnd":
                                res = {
                                    type: leftRes.type, // TODO handle cast
                                    value: leftRes.value & rightRes.value
                                }
                                break
                            case "ExclusiveOr":
                                res = {
                                    type: leftRes.type, // TODO handle cast
                                    value: leftRes.value ^ rightRes.value
                                }
                                break
                            case "InclusiveOr":
                                res = {
                                    type: leftRes.type, // TODO handle cast
                                    value: leftRes.value | rightRes.value
                                }
                                break
                            case "LogicalAnd":
                                res = {
                                    type: leftRes.type, // TODO handle cast
                                    value: leftRes.value && rightRes.value
                                }
                                break
                            case "LogicalOr":
                                res = {
                                    type: leftRes.type, // TODO handle cast
                                    value: leftRes.value || rightRes.value
                                }
                                break
                            case "FetchRHS":
                                throw new Error(`FetchRHS should not appear at frontend`)
                            default:
                                throw new Error(`${binaryOperator} not implemented`)
                        }
                        break
                    case "UnaryExpression":
                        const [unaryOperator, unaryExpression] = node[x]
                        res = visitNode([unaryExpression])
                        switch (unaryOperator) {
                            case "UnaryMinus":
                                res.value *= -1
                                break
                            case "BitwiseComplement":
                                res.value = ~res.value
                                break
                            case "LogicalNot":
                                res.value = !res.value
                                break
                            default:
                                throw new Error(`${unaryOperator} not implemented`)
                        }
                        break
                    case "CastExpression":
                        const [castType, castExpression] = node[x]
                        console.log(castType)
                        console.log(castExpression)
                        res = castValue(visitNode([castExpression]), castType)
                        // throw new Error(`${x} not implemented`)
                        break

                    // functions
                    case "Function":
                        const [retType, funcName, parameters, funcChildren] = node[x]
                        symbolTable.slice(-1)[0][funcName] = {
                            retType,
                            parameters,
                            funcBlock: funcChildren
                        }
                        break
                    case "ReturnStatement":
                        res = visitNode([node[x]])
                        callStack.slice(-1)[0].returnResult = res
                        callStack.slice(-1)[0].hasReturn = true
                        return res // [!] break current exec flow
                    case "FunctionCall":
                        const [funcCallName, funcArguments] = node[x]
                        if (funcCallName === "printf") {
                            const printfArgs = funcArguments.slice(1)
                            const args = printfArgs.map((arg) => {
                                return visitNode([arg]).value
                            })
                            stdout += vsprintf(visitNode([funcArguments[0]]).value, args)
                        } else {
                            const callee = findSymbol(funcCallName)
                            // put arguments into symbol table
                            symbolTable.push({})
                            callStack.push({
                                funcCallName,
                                funcArguments,
                                returnResult: null,
                                hasReturn: false,
                                loopStack: []
                            })
                            // eslint-disable-next-line array-callback-return
                            callee.parameters.map((parameter, i) => {
                                const {param_type, param_name} = parameter
                                symbolTable.slice(-1)[0][param_name] = castValue(visitNode([funcArguments[i]]), param_type)
                            })
                            visitNode(callee.funcBlock)
                            res = castValue(callStack.slice(-1)[0].returnResult, callee.retType)
                            callStack.pop()
                            symbolTable.pop()
                        }
                        break

                    // flow controls & statements
                    case "ForStatement":
                        const [initClause, condition, iteration, forStatement] = node[x]
                        const forLoopStack = callStack.slice(-1)[0].loopStack
                        forLoopStack.push(false)
                        for (visitNode(initClause); !forLoopStack.slice(-1)[0] && visitNode([condition]).value; visitNode([iteration])) {
                            visitNode([forStatement])
                        }
                        forLoopStack.pop()
                        break
                    case "WhileStatement":
                        const whileLoopStack = callStack.slice(-1)[0].loopStack
                        const [isDoWhile, whileCondition, whileStatements] = node[x]
                        whileLoopStack.push(false)
                        if (isDoWhile) {
                            do {
                                visitNode([whileStatements])
                            } while (!whileLoopStack.slice(-1)[0] && visitNode([whileCondition]).value)
                        } else {
                            while (!whileLoopStack.slice(-1)[0] && visitNode([whileCondition]).value) {
                                visitNode([whileStatements])
                            }
                        }
                        whileLoopStack.pop()
                        break
                    case "IfStatement":
                        const [ifCondExpression, ifTrueStatement, elseStatement] = node[x]
                        const ifRes = visitNode([ifCondExpression])
                        if (ifRes.value) {
                            visitNode([ifTrueStatement])
                        } else {
                            visitNode([elseStatement])
                        }
                        break
                    case "InitClause": // InitClause is a special form of BlockStatement
                    case "Arguments": // FuncCallArguments is a special form of BlockStatements
                    case "BlockStatement":
                        visitNode(node[x])
                        break

                    // unhandled case
                    default:
                        console.log(x)
                        console.log(node[x])
                        throw new Error(`${x} not implemented`)
                }
            }
        }
        return res
    }

    // scan global symbols
    visitNode(ast)

    // call entry function
    const funcCall = [{
        "FunctionCall": [
            entryPoint,
            args
        ]
    }]
    let res = visitNode(funcCall)
    res.stdout = stdout
    return res
}

// const res = evalAST(astData, 'main', [])
// console.log(JSON.stringify(res, null, 2))