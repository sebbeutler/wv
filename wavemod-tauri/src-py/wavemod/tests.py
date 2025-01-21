from .node import (
    Node,
    CoordinateMapping
)

def _test():
    print("wavemod.test")

def test_coordinate_mapping_default():
    default_mapping = CoordinateMapping.default()
    assert default_mapping.origin == (0., 0.)
    assert default_mapping.inverse_axis == (False, False)
    assert default_mapping.scale_axis == (1., 1.)

def test_coordinate_mapping_map_same():
    coords = (1., 2.)
    mapping = CoordinateMapping.default()
    assert CoordinateMapping.map(coords, mapping, mapping) == coords

def test_coordinate_mapping_map_different():
    coords = (1., 2.)
    current_mapping = CoordinateMapping(
        origin=(0., 0.),
        inverse_axis=(False, False),
        scale_axis=(1., 1.)
    )
    target_mapping = CoordinateMapping(
        origin=(1., 1.),
        inverse_axis=(True, False),
        scale_axis=(2., 1.)
    )
    mapped_coords = CoordinateMapping.map(coords, current_mapping, target_mapping)
    assert mapped_coords == (-1., 3.)