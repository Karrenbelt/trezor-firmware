# Automatically generated by pb2py
import protobuf as p


class EthereumSignTx(p.MessageType):
    FIELDS = {
        1: ('address_n', p.UVarintType, p.FLAG_REPEATED),
        2: ('nonce', p.BytesType, 0),
        3: ('gas_price', p.BytesType, 0),
        4: ('gas_limit', p.BytesType, 0),
        5: ('to', p.BytesType, 0),
        6: ('value', p.BytesType, 0),
        7: ('data_initial_chunk', p.BytesType, 0),
        8: ('data_length', p.UVarintType, 0),
        9: ('chain_id', p.UVarintType, 0),
    }
    MESSAGE_WIRE_TYPE = 58

    def __init__(
        self,
        address_n: list = [],
        nonce: bytes = None,
        gas_price: bytes = None,
        gas_limit: bytes = None,
        to: bytes = None,
        value: bytes = None,
        data_initial_chunk: bytes = None,
        data_length: int = None,
        chain_id: int = None,
        **kwargs,
    ):
        self.address_n = address_n
        self.nonce = nonce
        self.gas_price = gas_price
        self.gas_limit = gas_limit
        self.to = to
        self.value = value
        self.data_initial_chunk = data_initial_chunk
        self.data_length = data_length
        self.chain_id = chain_id
        p.MessageType.__init__(self, **kwargs)
