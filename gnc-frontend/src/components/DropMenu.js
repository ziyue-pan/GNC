import {Fragment, useState} from 'react'
import {Listbox, Transition} from '@headlessui/react'
import {CheckIcon, SelectorIcon} from '@heroicons/react/solid'

function classNames(...classes) {
    return classes.filter(Boolean).join(' ')
}

export default function DropMenu(props) {
    const options = props.options
    const [selected, setSelected] = useState(props.defaultOption)

    const handleChange = (value) => {
        setSelected(value)
        const { onChange } = props
        onChange?.(value)
    }

    return (
        <Listbox value={selected} onChange={handleChange}>
            {({open}) => (
                <div className="relative">
                    <Listbox.Button
                        className={classNames(props.widthClass, "relative h-full bg-white border border-gray-300 rounded-lg shadow-sm pl-2 pr-12 py-2 text-left cursor-default focus:outline-none focus:ring-1 focus:ring-green-500 focus:border-green-500 sm:text-sm")}
                    >
                            <span className="flex items-center">
                                <span className="ml-3 block truncate">{selected.name}</span>
                            </span>
                        <span
                            className="ml-3 absolute inset-y-0 right-0 flex items-center pr-2 pointer-events-none">
                                <SelectorIcon className="h-5 w-5 text-gray-400" aria-hidden="true"/>
                            </span>
                    </Listbox.Button>
                    <Transition
                        show={open}
                        as={Fragment}
                        enter="transition ease-out duration-100"
                        enterFrom="transform opacity-0 scale-95"
                        enterTo="transform opacity-100 scale-100"
                        leave="transition ease-in duration-75"
                        leaveFrom="transform opacity-100 scale-100"
                        leaveTo="transform opacity-0 scale-95"
                    >
                        <Listbox.Options
                            static
                            className="absolute z-10 mt-1 w-full bg-white shadow-lg max-h-56 rounded-lg py-1 text-base ring-1 ring-black ring-opacity-5 overflow-auto focus:outline-none sm:text-sm"
                        >
                            {options.map((option) => (
                                <Listbox.Option
                                    key={option.id}
                                    className={({active}) =>
                                        classNames(
                                            active ? 'text-white bg-green-600' : 'text-gray-900',
                                            'cursor-default select-none relative py-2 pl-3 pr-9'
                                        )
                                    }
                                    value={option}
                                >
                                    {({selected, active}) => (
                                        <>
                                            <div className="flex items-center">
                                                <span className={classNames(selected ? 'font-semibold' : 'font-normal', 'ml-3 block truncate')}>{option.name}</span>
                                            </div>
                                            {selected ? (
                                                <span className={classNames(active ? 'text-white' : 'text-green-600',
                                                        'absolute inset-y-0 right-0 flex items-center pr-4'
                                                    )}
                                                >
                                                    <CheckIcon className="h-5 w-5" aria-hidden="true"/>
                                                </span>
                                            ) : null}
                                        </>
                                    )}
                                </Listbox.Option>
                            ))}
                        </Listbox.Options>
                    </Transition>
                </div>
            )}
        </Listbox>
    )
}
