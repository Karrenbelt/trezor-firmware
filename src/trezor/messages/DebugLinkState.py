# Automatically generated by pb2py
import protobuf as p
from .HDNodeType import HDNodeType


class DebugLinkState(p.MessageType):
    FIELDS = {
        1: ('layout', p.BytesType, 0),
        2: ('pin', p.UnicodeType, 0),
        3: ('matrix', p.UnicodeType, 0),
        4: ('mnemonic', p.UnicodeType, 0),
        5: ('node', HDNodeType, 0),
        6: ('passphrase_protection', p.BoolType, 0),
        7: ('reset_word', p.UnicodeType, 0),
        8: ('reset_entropy', p.BytesType, 0),
        9: ('recovery_fake_word', p.UnicodeType, 0),
        10: ('recovery_word_pos', p.UVarintType, 0),
    }
    MESSAGE_WIRE_TYPE = 102

    def __init__(
        self,
        layout: bytes = None,
        pin: str = None,
        matrix: str = None,
        mnemonic: str = None,
        node: HDNodeType = None,
        passphrase_protection: bool = None,
        reset_word: str = None,
        reset_entropy: bytes = None,
        recovery_fake_word: str = None,
        recovery_word_pos: int = None,
        **kwargs,
    ):
        self.layout = layout
        self.pin = pin
        self.matrix = matrix
        self.mnemonic = mnemonic
        self.node = node
        self.passphrase_protection = passphrase_protection
        self.reset_word = reset_word
        self.reset_entropy = reset_entropy
        self.recovery_fake_word = recovery_fake_word
        self.recovery_word_pos = recovery_word_pos
        p.MessageType.__init__(self, **kwargs)
