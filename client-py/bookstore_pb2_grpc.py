# Generated by the gRPC Python protocol compiler plugin. DO NOT EDIT!
"""Client and server classes corresponding to protobuf-defined services."""
import grpc

import bookstore_pb2 as bookstore__pb2


class BookStoreStub(object):
    """Missing associated documentation comment in .proto file."""

    def __init__(self, channel):
        """Constructor.

        Args:
            channel: A grpc.Channel.
        """
        self.Hello = channel.unary_unary(
                '/bookstore.BookStore/Hello',
                request_serializer=bookstore__pb2.HelloRequest.SerializeToString,
                response_deserializer=bookstore__pb2.HelloReply.FromString,
                )
        self.Search = channel.unary_unary(
                '/bookstore.BookStore/Search',
                request_serializer=bookstore__pb2.BookTopicRequest.SerializeToString,
                response_deserializer=bookstore__pb2.BookListReply.FromString,
                )
        self.Lookup = channel.unary_unary(
                '/bookstore.BookStore/Lookup',
                request_serializer=bookstore__pb2.ItemNumberRequest.SerializeToString,
                response_deserializer=bookstore__pb2.BookInfoReply.FromString,
                )
        self.Buy = channel.unary_unary(
                '/bookstore.BookStore/Buy',
                request_serializer=bookstore__pb2.ItemNumberRequest.SerializeToString,
                response_deserializer=bookstore__pb2.BuyBookReply.FromString,
                )


class BookStoreServicer(object):
    """Missing associated documentation comment in .proto file."""

    def Hello(self, request, context):
        """Sends a greeting
        """
        context.set_code(grpc.StatusCode.UNIMPLEMENTED)
        context.set_details('Method not implemented!')
        raise NotImplementedError('Method not implemented!')

    def Search(self, request, context):
        """Search for books by topic
        """
        context.set_code(grpc.StatusCode.UNIMPLEMENTED)
        context.set_details('Method not implemented!')
        raise NotImplementedError('Method not implemented!')

    def Lookup(self, request, context):
        """Get book info by id
        """
        context.set_code(grpc.StatusCode.UNIMPLEMENTED)
        context.set_details('Method not implemented!')
        raise NotImplementedError('Method not implemented!')

    def Buy(self, request, context):
        """Buy a book by id
        """
        context.set_code(grpc.StatusCode.UNIMPLEMENTED)
        context.set_details('Method not implemented!')
        raise NotImplementedError('Method not implemented!')


def add_BookStoreServicer_to_server(servicer, server):
    rpc_method_handlers = {
            'Hello': grpc.unary_unary_rpc_method_handler(
                    servicer.Hello,
                    request_deserializer=bookstore__pb2.HelloRequest.FromString,
                    response_serializer=bookstore__pb2.HelloReply.SerializeToString,
            ),
            'Search': grpc.unary_unary_rpc_method_handler(
                    servicer.Search,
                    request_deserializer=bookstore__pb2.BookTopicRequest.FromString,
                    response_serializer=bookstore__pb2.BookListReply.SerializeToString,
            ),
            'Lookup': grpc.unary_unary_rpc_method_handler(
                    servicer.Lookup,
                    request_deserializer=bookstore__pb2.ItemNumberRequest.FromString,
                    response_serializer=bookstore__pb2.BookInfoReply.SerializeToString,
            ),
            'Buy': grpc.unary_unary_rpc_method_handler(
                    servicer.Buy,
                    request_deserializer=bookstore__pb2.ItemNumberRequest.FromString,
                    response_serializer=bookstore__pb2.BuyBookReply.SerializeToString,
            ),
    }
    generic_handler = grpc.method_handlers_generic_handler(
            'bookstore.BookStore', rpc_method_handlers)
    server.add_generic_rpc_handlers((generic_handler,))


 # This class is part of an EXPERIMENTAL API.
class BookStore(object):
    """Missing associated documentation comment in .proto file."""

    @staticmethod
    def Hello(request,
            target,
            options=(),
            channel_credentials=None,
            call_credentials=None,
            insecure=False,
            compression=None,
            wait_for_ready=None,
            timeout=None,
            metadata=None):
        return grpc.experimental.unary_unary(request, target, '/bookstore.BookStore/Hello',
            bookstore__pb2.HelloRequest.SerializeToString,
            bookstore__pb2.HelloReply.FromString,
            options, channel_credentials,
            insecure, call_credentials, compression, wait_for_ready, timeout, metadata)

    @staticmethod
    def Search(request,
            target,
            options=(),
            channel_credentials=None,
            call_credentials=None,
            insecure=False,
            compression=None,
            wait_for_ready=None,
            timeout=None,
            metadata=None):
        return grpc.experimental.unary_unary(request, target, '/bookstore.BookStore/Search',
            bookstore__pb2.BookTopicRequest.SerializeToString,
            bookstore__pb2.BookListReply.FromString,
            options, channel_credentials,
            insecure, call_credentials, compression, wait_for_ready, timeout, metadata)

    @staticmethod
    def Lookup(request,
            target,
            options=(),
            channel_credentials=None,
            call_credentials=None,
            insecure=False,
            compression=None,
            wait_for_ready=None,
            timeout=None,
            metadata=None):
        return grpc.experimental.unary_unary(request, target, '/bookstore.BookStore/Lookup',
            bookstore__pb2.ItemNumberRequest.SerializeToString,
            bookstore__pb2.BookInfoReply.FromString,
            options, channel_credentials,
            insecure, call_credentials, compression, wait_for_ready, timeout, metadata)

    @staticmethod
    def Buy(request,
            target,
            options=(),
            channel_credentials=None,
            call_credentials=None,
            insecure=False,
            compression=None,
            wait_for_ready=None,
            timeout=None,
            metadata=None):
        return grpc.experimental.unary_unary(request, target, '/bookstore.BookStore/Buy',
            bookstore__pb2.ItemNumberRequest.SerializeToString,
            bookstore__pb2.BuyBookReply.FromString,
            options, channel_credentials,
            insecure, call_credentials, compression, wait_for_ready, timeout, metadata)
