from google.protobuf.internal import containers as _containers
from google.protobuf import descriptor as _descriptor
from google.protobuf import message as _message
from typing import ClassVar as _ClassVar, Iterable as _Iterable, Mapping as _Mapping, Optional as _Optional, Union as _Union

DESCRIPTOR: _descriptor.FileDescriptor

class Book(_message.Message):
    __slots__ = ["id", "price", "stock", "title", "topic"]
    ID_FIELD_NUMBER: _ClassVar[int]
    PRICE_FIELD_NUMBER: _ClassVar[int]
    STOCK_FIELD_NUMBER: _ClassVar[int]
    TITLE_FIELD_NUMBER: _ClassVar[int]
    TOPIC_FIELD_NUMBER: _ClassVar[int]
    id: int
    price: float
    stock: int
    title: str
    topic: str
    def __init__(self, id: _Optional[int] = ..., title: _Optional[str] = ..., topic: _Optional[str] = ..., stock: _Optional[int] = ..., price: _Optional[float] = ...) -> None: ...

class BookInfoReply(_message.Message):
    __slots__ = ["book", "message", "success"]
    BOOK_FIELD_NUMBER: _ClassVar[int]
    MESSAGE_FIELD_NUMBER: _ClassVar[int]
    SUCCESS_FIELD_NUMBER: _ClassVar[int]
    book: Book
    message: str
    success: bool
    def __init__(self, success: bool = ..., message: _Optional[str] = ..., book: _Optional[_Union[Book, _Mapping]] = ...) -> None: ...

class BookListReply(_message.Message):
    __slots__ = ["books", "message", "success"]
    BOOKS_FIELD_NUMBER: _ClassVar[int]
    MESSAGE_FIELD_NUMBER: _ClassVar[int]
    SUCCESS_FIELD_NUMBER: _ClassVar[int]
    books: _containers.RepeatedCompositeFieldContainer[Book]
    message: str
    success: bool
    def __init__(self, success: bool = ..., message: _Optional[str] = ..., books: _Optional[_Iterable[_Union[Book, _Mapping]]] = ...) -> None: ...

class BookTopicRequest(_message.Message):
    __slots__ = ["topic"]
    TOPIC_FIELD_NUMBER: _ClassVar[int]
    topic: str
    def __init__(self, topic: _Optional[str] = ...) -> None: ...

class BuyBookReply(_message.Message):
    __slots__ = ["message", "success"]
    MESSAGE_FIELD_NUMBER: _ClassVar[int]
    SUCCESS_FIELD_NUMBER: _ClassVar[int]
    message: str
    success: bool
    def __init__(self, success: bool = ..., message: _Optional[str] = ...) -> None: ...

class HelloReply(_message.Message):
    __slots__ = ["hello_message"]
    HELLO_MESSAGE_FIELD_NUMBER: _ClassVar[int]
    hello_message: str
    def __init__(self, hello_message: _Optional[str] = ...) -> None: ...

class HelloRequest(_message.Message):
    __slots__ = ["name"]
    NAME_FIELD_NUMBER: _ClassVar[int]
    name: str
    def __init__(self, name: _Optional[str] = ...) -> None: ...

class ItemNumberRequest(_message.Message):
    __slots__ = ["id"]
    ID_FIELD_NUMBER: _ClassVar[int]
    id: int
    def __init__(self, id: _Optional[int] = ...) -> None: ...
