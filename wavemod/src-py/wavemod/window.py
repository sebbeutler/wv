from typing_extensions import Self
from .gpu import Renderer

class Window:
    def _rezize_window(self, window, width, height):
        pass

    def _center_window(self, window):
        pass

    def _window_renderer(self, window, label):
        pass
    
    def resize(self, width: int, height: int) -> Self:
        self._rezize_window(self._window, width, height)
        return self
    
    def center(self) -> Self:
        self._center_window(self._window)
        return self
    
    def get_renderer(self, label: str) -> Renderer:
        return self._window_renderer(self._window, label)