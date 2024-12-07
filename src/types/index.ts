interface Message {
    plain: string,
    html: string
}

interface Error {
    error_type: string,
    index: number,
    message: Message
}

interface Success {
    message: Message
}

export {Message, Success, Error};
