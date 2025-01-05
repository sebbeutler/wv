import wavemod as wmd
from wavemod import CURRENT_NODE as this
from math import cos, sin, pi, atan2
# SETUP
this.setSize(7000, 3000)

# E1

G = 3.711 # m/s² (accel)
thrust_force = 1. # [0,4] m/s² (accel)
thrust_angle = 0. # [-90°,90°] (dir)

# Conditions
MAX_VSPEED = 40. # m/s (speed)
MAX_HSPEED = 20. # m/s (speed)

Point = tuple[int, int]
PointF = tuple[float, float]

# World Mapping
surfaceN = int(input())
surface_points: list[Point] = []
land_zone: tuple[Point, Point] = None

for point in range(surfaceN):
    px, py = map(int, input().split())
    previous_point = surface_points[-1]
    if px == previous_point[0]:
        land_zone = (previous_point, (px, py))
    surface_points.append((px, py))

assert surface_points[0][0] == 0 and surface_points[-1][1] == 6999

# Get Coordinates
from dataclasses import dataclass
from typing_extensions import Self
@dataclass
class LanderState:
    # Context
    def from_str(raw: str) -> Self:
        return LanderState(*raw.split(' '))
    
    # Position
    pos_x: int
    pos_y: int
    @property
    def pos(self) -> PointF:
        return (self.pos_x, self.pos_y)
    
    # Speed
    speed_x: int
    speed_y: int
    @property
    def speed(self) -> PointF:
        return (self.speed_x, self.speed_y)
    
    # Lander State
    fuel: int
    rotate: int
    thrust: int
    
    # Lander Logic
    
    def navigate_to(self, target: PointF):
        pass
    
    def vertical_landing(self):
        if self.vSpeed > MAX_VSPEED:
            self.thrust = min(self.thrust+1, 4)
        elif self.vSpeed < 5:
            self.thrust = max(self.thrust+1, 2)

previous_state = None

def step(previous: LanderState, target: PointF):
    lander: LanderState = LanderState.from_str(input())
    
    g = (0., -G)
    speed = lander.speed
    rad = (3.*pi/2.) + (pi/2) * (lander.rotate/90.)
    
    thrust = (cos(rad) * lander.thrust, sin(rad)*lander.thrust)
    
    best = ()
    
    dist_x = target[0] - lander.pos_x
    dist_y = target[1] - lander.pos_y
    dist_a = atan2(dist_y, dist_x)
    T_p = 1
    T_a = dist_a
    
    lander.thrust = T_p
    lander.rotate = dist_a
    
    return lander

previous_state: LanderState = LanderState.from_raw(input())
target = (previous_state.pos_x, previous_state.pos_y)
output = print
output(f"dir:{previous_state.rotate} thrust:{previous_state.thrust}")

while True:
    previous_state = step(previous_state, target)
    output(f"dir:{previous_state.rotate} thrust:{previous_state.thrust}")


