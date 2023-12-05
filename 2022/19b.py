import dataclasses
import re

import numpy as np
import cvxpy as cp

BLUEPRINT_REGEX = "Blueprint ([0-9]+): Each ore robot costs ([0-9]+) ore. Each clay robot costs ([0-9]+) ore. Each obsidian robot costs ([0-9]+) ore and ([0-9]+) clay. Each geode robot costs ([0-9]+) ore and ([0-9]+) obsidian."


@dataclasses.dataclass
class Blueprint:
    index: int
    ore_robot_cost_ore: int
    clay_robot_cost_ore: int
    obsidian_robot_cost_ore: int
    obsidian_robot_cost_clay: int
    geode_robot_cost_ore: int
    geode_robot_cost_obsidian: int


def parse(input_path):
    input = open(input_path, "r")
    input = input.read()

    matches = re.findall(BLUEPRINT_REGEX, input)

    blueprints = [
        Blueprint(
            index=int(a),
            ore_robot_cost_ore=int(b),
            clay_robot_cost_ore=int(c),
            obsidian_robot_cost_ore=int(d),
            obsidian_robot_cost_clay=int(e),
            geode_robot_cost_ore=int(f),
            geode_robot_cost_obsidian=int(g),
        )
        for a, b, c, d, e, f, g in matches
    ]

    return blueprints


def max_geodes(blueprint, T):
    constraints = []

    build_ore_robot = cp.Variable(T + 1, boolean=True)
    build_clay_robot = cp.Variable(T + 1, boolean=True)
    build_obsidian_robot = cp.Variable(T + 1, boolean=True)
    build_geode_robot = cp.Variable(T + 1, boolean=True)

    constraints.append(build_ore_robot[0] == 0)
    constraints.append(build_clay_robot[0] == 0)
    constraints.append(build_obsidian_robot[0] == 0)
    constraints.append(build_geode_robot[0] == 0)

    for t in range(1, T + 1):
        elapsed = np.flip(np.arange(t))

        # Ore production and use
        constraints.append(
            t  # Start with one ore robot
            + cp.sum(elapsed * build_ore_robot[:t])
            - blueprint.ore_robot_cost_ore * cp.sum(build_ore_robot[: t + 1])
            - blueprint.clay_robot_cost_ore * cp.sum(build_clay_robot[: t + 1])
            - blueprint.obsidian_robot_cost_ore * cp.sum(build_obsidian_robot[: t + 1])
            - blueprint.geode_robot_cost_ore * cp.sum(build_geode_robot[: t + 1])
            >= 0
        )

        # Clay production and use
        constraints.append(
            cp.sum(elapsed * build_clay_robot[:t])
            - blueprint.obsidian_robot_cost_clay * cp.sum(build_obsidian_robot[: t + 1])
            >= 0
        )

        # Obsidian production and use
        constraints.append(
            cp.sum(elapsed * build_obsidian_robot[:t])
            - blueprint.geode_robot_cost_obsidian * cp.sum(build_geode_robot[: t + 1])
            >= 0
        )

        # Geode production and use
        constraints.append(cp.sum(elapsed * build_geode_robot[:t]) >= 0)

        # Only one robot can be constructed per minute.
        constraints.append(
            build_ore_robot[t]
            + build_clay_robot[t]
            + build_obsidian_robot[t]
            + build_geode_robot[t]
            <= 1
        )

    final_geodes = cp.sum(np.flip(np.arange(T)) * build_geode_robot[:T])
    problem = cp.Problem(cp.Maximize(final_geodes), constraints)

    max_final_geodes = problem.solve(verbose=True)

    print(max_final_geodes)
    return int(max_final_geodes)


def run(input_path):
    T = 32
    blueprints = parse(input_path)

    prod_max = 1
    for blueprint in blueprints[:3]:
        max_final_geodes = max_geodes(blueprint, T)
        prod_max *= max_final_geodes

    print(prod_max)
    return prod_max


if __name__ == "__main__":
    assert run("19-real.txt") == 5800
