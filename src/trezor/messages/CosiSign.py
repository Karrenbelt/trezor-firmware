# Automatically generated by pb2py
import protobuf as p


class CosiSign(p.MessageType):
    FIELDS = {
        1: ('address_n', p.UVarintType, p.FLAG_REPEATED),
        2: ('data', p.BytesType, 0),
        3: ('global_commitment', p.BytesType, 0),
        4: ('global_pubkey', p.BytesType, 0),
    }
    MESSAGE_WIRE_TYPE = 73

    def __init__(
        self,
        address_n: list = [],
        data: bytes = None,
        global_commitment: bytes = None,
        global_pubkey: bytes = None,
        **kwargs,
    ):
        self.address_n = address_n
        self.data = data
        self.global_commitment = global_commitment
        self.global_pubkey = global_pubkey
        p.MessageType.__init__(self, **kwargs)
