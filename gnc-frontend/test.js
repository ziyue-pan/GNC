// import {sprintf} from 'sprintf-js'
const vsprintf = require('sprintf-js').vsprintf;

/***
 * Mocked Data
 */
const data = {"error":false,"error_message":"","parse_tree":{"id":"gnc(0,94)","label":"gnc","children":[{"id":"function(0,94)","label":"function","children":[{"id":"data_type(0,4)","label":"data_type","children":[{"id":"int(0,3)","label":"int","children":[]}]},{"id":"identifier(4,8)","label":"identifier","children":[]},{"id":"function_parameter_list(8,10)","label":"function_parameter_list","children":[]},{"id":"statement(17,46)","label":"statement","children":[{"id":"declaration_statement(17,45)","label":"declaration_statement","children":[{"id":"data_type(17,22)","label":"data_type","children":[{"id":"char(17,21)","label":"char","children":[]},{"id":"star(21,22)","label":"star","children":[]}]},{"id":"identifier(23,24)","label":"identifier","children":[]},{"id":"expression(27,45)","label":"expression","children":[{"id":"logical_or_expression(27,45)","label":"logical_or_expression","children":[{"id":"logical_and_expression(27,45)","label":"logical_and_expression","children":[{"id":"inclusive_or_expression(27,45)","label":"inclusive_or_expression","children":[{"id":"exclusive_or_expression(27,45)","label":"exclusive_or_expression","children":[{"id":"bitwise_and_expression(27,45)","label":"bitwise_and_expression","children":[{"id":"equality_expression(27,45)","label":"equality_expression","children":[{"id":"comparison_expression(27,45)","label":"comparison_expression","children":[{"id":"shift_expression(27,45)","label":"shift_expression","children":[{"id":"additive_expression(27,45)","label":"additive_expression","children":[{"id":"multiplicative_expression(27,45)","label":"multiplicative_expression","children":[{"id":"cast_expression(27,45)","label":"cast_expression","children":[{"id":"unary_expression(27,45)","label":"unary_expression","children":[{"id":"string_literal(27,45)","label":"string_literal","children":[{"id":"str_non_escape(28,32)","label":"str_non_escape","children":[]},{"id":"str_escape(32,34)","label":"str_escape","children":[{"id":"char_escape(33,34)","label":"char_escape","children":[]}]},{"id":"str_escape(34,36)","label":"str_escape","children":[{"id":"char_escape(35,36)","label":"char_escape","children":[]}]},{"id":"str_escape(36,38)","label":"str_escape","children":[{"id":"char_escape(37,38)","label":"char_escape","children":[]}]},{"id":"str_escape(38,42)","label":"str_escape","children":[{"id":"hex_escape(39,42)","label":"hex_escape","children":[]}]},{"id":"str_escape(42,44)","label":"str_escape","children":[{"id":"char_escape(43,44)","label":"char_escape","children":[]}]}]}]}]}]}]}]}]}]}]}]}]}]}]}]}]}]},{"id":"statement(52,77)","label":"statement","children":[{"id":"expression(52,76)","label":"expression","children":[{"id":"logical_or_expression(52,76)","label":"logical_or_expression","children":[{"id":"logical_and_expression(52,76)","label":"logical_and_expression","children":[{"id":"inclusive_or_expression(52,76)","label":"inclusive_or_expression","children":[{"id":"exclusive_or_expression(52,76)","label":"exclusive_or_expression","children":[{"id":"bitwise_and_expression(52,76)","label":"bitwise_and_expression","children":[{"id":"equality_expression(52,76)","label":"equality_expression","children":[{"id":"comparison_expression(52,76)","label":"comparison_expression","children":[{"id":"shift_expression(52,76)","label":"shift_expression","children":[{"id":"additive_expression(52,76)","label":"additive_expression","children":[{"id":"multiplicative_expression(52,76)","label":"multiplicative_expression","children":[{"id":"cast_expression(52,76)","label":"cast_expression","children":[{"id":"unary_expression(52,76)","label":"unary_expression","children":[{"id":"function_call(52,76)","label":"function_call","children":[{"id":"identifier(52,58)","label":"identifier","children":[]},{"id":"expression(59,63)","label":"expression","children":[{"id":"logical_or_expression(59,63)","label":"logical_or_expression","children":[{"id":"logical_and_expression(59,63)","label":"logical_and_expression","children":[{"id":"inclusive_or_expression(59,63)","label":"inclusive_or_expression","children":[{"id":"exclusive_or_expression(59,63)","label":"exclusive_or_expression","children":[{"id":"bitwise_and_expression(59,63)","label":"bitwise_and_expression","children":[{"id":"equality_expression(59,63)","label":"equality_expression","children":[{"id":"comparison_expression(59,63)","label":"comparison_expression","children":[{"id":"shift_expression(59,63)","label":"shift_expression","children":[{"id":"additive_expression(59,63)","label":"additive_expression","children":[{"id":"multiplicative_expression(59,63)","label":"multiplicative_expression","children":[{"id":"cast_expression(59,63)","label":"cast_expression","children":[{"id":"unary_expression(59,63)","label":"unary_expression","children":[{"id":"string_literal(59,63)","label":"string_literal","children":[{"id":"str_non_escape(60,62)","label":"str_non_escape","children":[]}]}]}]}]}]}]}]}]}]}]}]}]}]}]},{"id":"expression(65,66)","label":"expression","children":[{"id":"logical_or_expression(65,66)","label":"logical_or_expression","children":[{"id":"logical_and_expression(65,66)","label":"logical_and_expression","children":[{"id":"inclusive_or_expression(65,66)","label":"inclusive_or_expression","children":[{"id":"exclusive_or_expression(65,66)","label":"exclusive_or_expression","children":[{"id":"bitwise_and_expression(65,66)","label":"bitwise_and_expression","children":[{"id":"equality_expression(65,66)","label":"equality_expression","children":[{"id":"comparison_expression(65,66)","label":"comparison_expression","children":[{"id":"shift_expression(65,66)","label":"shift_expression","children":[{"id":"additive_expression(65,66)","label":"additive_expression","children":[{"id":"multiplicative_expression(65,66)","label":"multiplicative_expression","children":[{"id":"cast_expression(65,66)","label":"cast_expression","children":[{"id":"unary_expression(65,66)","label":"unary_expression","children":[{"id":"identifier(65,66)","label":"identifier","children":[]}]}]}]}]}]}]}]}]}]}]}]}]}]},{"id":"expression(68,69)","label":"expression","children":[{"id":"logical_or_expression(68,69)","label":"logical_or_expression","children":[{"id":"logical_and_expression(68,69)","label":"logical_and_expression","children":[{"id":"inclusive_or_expression(68,69)","label":"inclusive_or_expression","children":[{"id":"exclusive_or_expression(68,69)","label":"exclusive_or_expression","children":[{"id":"bitwise_and_expression(68,69)","label":"bitwise_and_expression","children":[{"id":"equality_expression(68,69)","label":"equality_expression","children":[{"id":"comparison_expression(68,69)","label":"comparison_expression","children":[{"id":"shift_expression(68,69)","label":"shift_expression","children":[{"id":"additive_expression(68,69)","label":"additive_expression","children":[{"id":"multiplicative_expression(68,69)","label":"multiplicative_expression","children":[{"id":"cast_expression(68,69)","label":"cast_expression","children":[{"id":"unary_expression(68,69)","label":"unary_expression","children":[{"id":"int_literal(68,69)","label":"int_literal","children":[{"id":"dec_literal(68,69)","label":"dec_literal","children":[]}]}]}]}]}]}]}]}]}]}]}]}]}]}]},{"id":"expression(71,72)","label":"expression","children":[{"id":"logical_or_expression(71,72)","label":"logical_or_expression","children":[{"id":"logical_and_expression(71,72)","label":"logical_and_expression","children":[{"id":"inclusive_or_expression(71,72)","label":"inclusive_or_expression","children":[{"id":"exclusive_or_expression(71,72)","label":"exclusive_or_expression","children":[{"id":"bitwise_and_expression(71,72)","label":"bitwise_and_expression","children":[{"id":"equality_expression(71,72)","label":"equality_expression","children":[{"id":"comparison_expression(71,72)","label":"comparison_expression","children":[{"id":"shift_expression(71,72)","label":"shift_expression","children":[{"id":"additive_expression(71,72)","label":"additive_expression","children":[{"id":"multiplicative_expression(71,72)","label":"multiplicative_expression","children":[{"id":"cast_expression(71,72)","label":"cast_expression","children":[{"id":"unary_expression(71,72)","label":"unary_expression","children":[{"id":"int_literal(71,72)","label":"int_literal","children":[{"id":"dec_literal(71,72)","label":"dec_literal","children":[]}]}]}]}]}]}]}]}]}]}]}]}]}]}]},{"id":"expression(74,75)","label":"expression","children":[{"id":"logical_or_expression(74,75)","label":"logical_or_expression","children":[{"id":"logical_and_expression(74,75)","label":"logical_and_expression","children":[{"id":"inclusive_or_expression(74,75)","label":"inclusive_or_expression","children":[{"id":"exclusive_or_expression(74,75)","label":"exclusive_or_expression","children":[{"id":"bitwise_and_expression(74,75)","label":"bitwise_and_expression","children":[{"id":"equality_expression(74,75)","label":"equality_expression","children":[{"id":"comparison_expression(74,75)","label":"comparison_expression","children":[{"id":"shift_expression(74,75)","label":"shift_expression","children":[{"id":"additive_expression(74,75)","label":"additive_expression","children":[{"id":"multiplicative_expression(74,75)","label":"multiplicative_expression","children":[{"id":"cast_expression(74,75)","label":"cast_expression","children":[{"id":"unary_expression(74,75)","label":"unary_expression","children":[{"id":"int_literal(74,75)","label":"int_literal","children":[{"id":"dec_literal(74,75)","label":"dec_literal","children":[]}]}]}]}]}]}]}]}]}]}]}]}]}]}]}]}]}]}]}]}]}]}]}]}]}]}]}]}]}]},{"id":"statement(83,92)","label":"statement","children":[{"id":"return_statement(83,91)","label":"return_statement","children":[{"id":"expression(90,91)","label":"expression","children":[{"id":"logical_or_expression(90,91)","label":"logical_or_expression","children":[{"id":"logical_and_expression(90,91)","label":"logical_and_expression","children":[{"id":"inclusive_or_expression(90,91)","label":"inclusive_or_expression","children":[{"id":"exclusive_or_expression(90,91)","label":"exclusive_or_expression","children":[{"id":"bitwise_and_expression(90,91)","label":"bitwise_and_expression","children":[{"id":"equality_expression(90,91)","label":"equality_expression","children":[{"id":"comparison_expression(90,91)","label":"comparison_expression","children":[{"id":"shift_expression(90,91)","label":"shift_expression","children":[{"id":"additive_expression(90,91)","label":"additive_expression","children":[{"id":"multiplicative_expression(90,91)","label":"multiplicative_expression","children":[{"id":"cast_expression(90,91)","label":"cast_expression","children":[{"id":"unary_expression(90,91)","label":"unary_expression","children":[{"id":"int_literal(90,91)","label":"int_literal","children":[{"id":"dec_literal(90,91)","label":"dec_literal","children":[]}]}]}]}]}]}]}]}]}]}]}]}]}]}]}]}]}]},{"id":"EOI(94,94)","label":"EOI","children":[]}]},"ast":[{"Function":["Int","main",[],[{"Declaration":[{"Pointer":"Char"},"s"]},{"Assignment":["Simple",{"Identifier":"s"},{"StringLiteral":"fuck\\\t\n\u000f'"}]},{"FunctionCall":["printf",[{"StringLiteral":"%s"},{"Identifier":"s"},{"IntLiteral":1},{"IntLiteral":2},{"IntLiteral":3}]]},{"ReturnStatement":{"IntLiteral":0}}]]}]}
const astData = data.ast

/***
 * HLVM - High Level Virtual Machine
 * @param ast AST to be interpreted
 * @param entryPoint entry function name
 * @param args arguments as AST, e.g. {"IntLiteral": 233}
 * @returns {type, value} result
 */
function evalAST(ast, entryPoint, args) {
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
                if (origData.type === "Bool") {
                    res.value = origData.value
                } else {
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

const res = evalAST(astData, 'main', [])
console.log(JSON.stringify(res, null, 2))