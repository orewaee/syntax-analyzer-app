interface Message {
    plain: string,
    html: string
}

interface Error {
    index: number,
    message: Message
}

interface Success {
    semantics?: string,
    message: Message
}

export {Message, Success, Error};
