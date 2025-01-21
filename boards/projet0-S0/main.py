"""
Axiomes:
    - Identité: A = A
    - Non-contradiction: ¬(A & ¬A)
    - Tiers Exclu: A || ¬A
    - Quantum Consistency :
        A = A => A + C = A + D => C = D
        A ≠ A => A + C ≠ A + D => C ≠ D
"""
# >!connect /Users/sb/Source/wavemod/pinboard/boards/projet0-S0/main.py

from dataclasses import dataclass
from typing import Union, Set, List

class MultiSet:
    def __init__(self, elements: List[Union[int, float, bool]] = None):
        self.elements = elements if elements is not None else []
        self.members = set(self.elements)

    @staticmethod
    def from_repeat(start: int, finish: int, members: Set[Union[int, float, bool]] = {0}) -> 'MultiSet':
        if finish == float('inf'):
            finish = 1000
        elements = list(members) * (finish - start)
        return MultiSet(elements)
    
    def copy(self) -> 'MultiSet':
        return MultiSet(self.elements.copy())
    
    def __add__(self, other: Union[int, float, 'MultiSet']) -> 'MultiSet':
        if isinstance(other, (int, float)):
            new_elements = self.elements + [other] * len(self.elements)
            return MultiSet(new_elements)
        elif isinstance(other, MultiSet):
            new_elements = self.elements + other.elements
            return MultiSet(new_elements)
        else:
            raise TypeError(f"Unsupported operand type(s) for +: 'MultiSet' and '{type(other).__name__}'")

@dataclass
class PlanckObject:
    Length: str = ""
    Time: str = ""
    Mass: str = ""
    Energy: str = ""
    Charge: str = ""
    Temperature: str = ""
    Particle: str = ""

class Univers:
    def __init__(self):
        self.Constants = PlanckObject()
        self.multiset = MultiSet()
        self.operation_stack: List[tuple] = []
    
    def __add__(self, other: Union[int, float]) -> 'Univers':
        if isinstance(other, (int, float)):
            return self.copy().queue((wany, self.multiset, other))
        else:
            raise TypeError(f"Unsupported operand type(s) for +: 'Univers' and '{type(other).__name__}'")
    
    def queue(self, op: tuple) -> 'Univers':
        self.operation_stack.append(op)
        return self

    def copy(self) -> 'Univers':
        u = Univers()
        u.Constants = self.Constants
        u.multiset = self.multiset.copy()
        u.operation_stack = self.operation_stack.copy()
        return u

def wany(e: MultiSet, amount: Union[int, float]):
    pass

def ε(S: Univers) -> float:
    return 0.0

S0 = MultiSet.from_repeat(1, float('inf'), {True, False})

𝛿 = 1  # magnitude
ϕ = 1  # disturbance

S1 = S0 + 𝛿 * ϕ

assert ε(S1) == 𝛿 * ϕ
