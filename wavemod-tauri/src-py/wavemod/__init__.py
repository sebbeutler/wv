# print("Loaded wavemod pylib")

import wavemod_rs as rust # type: ignore

from . import utils
from .gpu import (Renderer, Pipeline, Buffer)
from .node import Node
from .window import Window
from . import tests

this: Node = None