class TestClass:
    class TestNestedClass:
        def test_nestedclass_method(self):
            assert 2 == 2

    def test_method(self):
        assert 1 == 1


class TestClassObj(object):
    def test_method_obj(self):
        assert 1 == 1


def test_function():
    assert 1 == 1


async def test_async_function():
    assert 1 == 1
