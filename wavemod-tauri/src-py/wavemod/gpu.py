from typing_extensions import Self

class Buffer:
    pass

class Pipeline:
    def create_buffer(self, *args) -> Buffer:
        self._create_buffer(self._pipeline, *args)

    def _create_buffer(self, pipeline, *args):
        pass

class Renderer:
    def create_pipeline(self, source: str) -> Pipeline:
        return self._create_pipeline(self._texture, source)
    
    def _create_pipeline(self, texture, source):
        pass

