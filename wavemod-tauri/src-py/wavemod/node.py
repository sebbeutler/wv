from dataclasses import dataclass
import wavemod_rs as wmd_rs # type: ignore

from typing_extensions import Self
from typing import Tuple

Coordinate = Tuple[float, ...]

@dataclass
class CoordinateMapping:
    origin: Coordinate
    inverse_axis: Tuple[bool, ...]
    scale_axis: Tuple[bool, ...]

    @staticmethod
    def default() -> Self:
        return CoordinateMapping(
            origin=(0., 0.),
            inverse_axis=(False, False),
            scale_axis=(1., 1.)
        )
    
    @staticmethod
    def map(coords: Coordinate, current_mapping: Self, target_mapping: Self) -> Self:
        if current_mapping == target_mapping:
            return coords

        assert len(coords) == len(current_mapping.origin) == len(current_mapping.inverse_axis) == len(current_mapping.scale_axis) == len(target_mapping.origin) == len(target_mapping.inverse_axis) == len(target_mapping.scale_axis), "Length mismatch in coordinate mappings"
        new_coords = []
        # Align axes direction
        for (axis, (cur_inv, tar_inv)) in enumerate(zip(current_mapping.inverse_axis, target_mapping.inverse_axis)):
            new_coords.append(-coords[axis] if cur_inv != tar_inv else coords[axis])
        
        # Calculate mapping difference
        origin_shift = target_mapping.origin - current_mapping.origin
        scale_shift = target_mapping.scale_axis * current_mapping.scale_axis
        
        # Apply the new mapping
        for axis in range(len(coords)):
            new_coords[axis] *= scale_shift[axis]
            new_coords[axis] += origin_shift[axis]
        
        return tuple(new_coords)
        

DEFAULT_MAPPING = CoordinateMapping.default()

class Node():
    def __init__(
        self,
        pronode: Self=None,
        coords: tuple[float, ...]=(1., 1.),
        size: tuple[float, ...]=(25., 25.),
        mapping: CoordinateMapping=CoordinateMapping.default(),
    ):
        self.id = wmd_rs.create_node(pronode.id if pronode else 0)
        self.pronode = pronode
        self.mapping = mapping
        self.resize(size)
        self.move(coords)
    
    def center(self):
        if pronode := self.pronode():
            px, py = pronode.coordinates()
            pw, ph = pronode.size()
            w, h = CoordinateMapping.map(self.size(), self.mapping, pronode.mapping)
            x = px + (pw - w) / 2.
            y = py + (ph - h) / 2.
            self.move((x, y), mapping=pronode.mapping())
        else:
            print("Warning: Cannot center node with missing pronode")
    
    def size(self) -> Coordinate:
        pass
    
    def resize(self, width, height):
        pass
    
    def coordinates(self) -> Coordinate:
        pass
    
    def move(self, coords: Coordinate, mapping: CoordinateMapping=DEFAULT_MAPPING):
        pass
    
    def mapping(self) -> CoordinateMapping:
        return self.mapping

