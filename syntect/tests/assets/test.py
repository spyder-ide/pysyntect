
import package
from package import (Class1, Class2, func_1, func_2)
from package3 import *


class TestClass(BaseClass1, BaseClass2):
    def __init__(self, x: int, y: List[Union[None, str]], z='default',
                 *args, **kwargs):
        super().__init__()
        self.x = x  # type: int

    def method1(self):
        pass


@decorator
async def test2(x, y, z):
    async for i in x:
        yield y
    with open('text', 'r', encoding='utf-8') as f:
        while True:
            x += 1
