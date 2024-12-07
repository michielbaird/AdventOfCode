import re
import sys
from dataclasses import dataclass, replace, astuple
from typing import Tuple
from functools import reduce
import argparse


@dataclass(frozen=True)
class Blueprint:
    id: int
    ore_cost: int
    clay_cost: int
    obsidian_cost: Tuple[int, int]
    geode_cost: Tuple[int, int]

@dataclass(frozen=True)
class DpState:
    ore: int
    clay: int
    obsidian: int
    robotOre: int
    robotClay: int
    robotObsidian: int
    robotGeode: int
    geodes: int


@dataclass(frozen=True)
class BuildState:
    ore: int
    clay: int
    obsidian: int
    robotOre: int
    robotClay: int
    robotObsidian: int
    robotGeode: int
    geodes: int
    def advance(
            self, 
            oreCost=0, 
            clayCost=0, 
            obsidianCost=0, 
            robotOreDelta=0, 
            robotClayDelta=0, 
            robotObsidianDelta=0,
            robotGeodeDelta=0
    ):
        return replace(self,
            ore = self.ore + self.robotOre - oreCost, 
            clay = self.clay + self.robotClay - clayCost,
            obsidian = self.obsidian + self.robotObsidian - obsidianCost,
            geodes = self.geodes + self.robotGeode,
            robotOre = self.robotOre + robotOreDelta,
            robotClay = self.robotClay + robotClayDelta,
            robotObsidian = self.robotObsidian + robotObsidianDelta,
            robotGeode = self.robotGeode + robotGeodeDelta 
        )

TOTAL_TIME = 32

p = re.compile(r"Blueprint (\d+): Each ore robot costs (\d+) ore. Each clay robot costs (\d+) ore. Each obsidian robot costs (\d+) ore and (\d+) clay. Each geode robot costs (\d+) ore and (\d+) obsidian.")
def decode_blueprint(raw):
    m = p.match(raw)
    return Blueprint(
        int(m.group(1)),
        int(m.group(2)),
        int(m.group(3)),
        (int(m.group(4)), int(m.group(5))),
        (int(m.group(6)), int(m.group(7)))
    )

def solve_blueprint(blueprint: Blueprint, total_time):
    # State:
    # (ore, clay, obsidian, robotOre, robotClay, robotObsidian) -> max(robotGeode)
    max_ore = max(blueprint.clay_cost, blueprint.ore_cost, blueprint.obsidian_cost[0], blueprint.geode_cost[0])
    max_clay = blueprint.obsidian_cost[1]
    max_obsidian = blueprint.geode_cost[1]
    #rows = []
    best = 0
    dp = {DpState(0,0,0,1,0,0,0,0): 0}
    for t in range(total_time):
        next_dp = {}
        max_spend_ore = (total_time - t - 1)*(max_ore)
        max_spend_clay = (total_time - t - 1)*(max_clay)
        max_spend_obsidian = (total_time - t -1)*(max_obsidian)
        print(str(t), end=", ", flush=True)
        rem_t = total_time - t - 1

        for state, geodes in dp.items():
            next_state = replace(
                state,
                ore=min(state.ore + state.robotOre, max_spend_ore),
                clay=min(state.clay + state.robotClay, max_spend_clay),
                obsidian=min(state.obsidian + state.robotObsidian, max_spend_obsidian),
            )

            next_geodes = geodes + state.robotGeode
            best = max(best, next_geodes)
            best_possible = next_geodes + rem_t*state.robotGeode + (rem_t*(rem_t-1))//2
            if best > best_possible:
                continue

            next_dp[next_state] = max(next_dp.get(next_state, -1), next_geodes)
            if state.robotOre < max_ore and state.ore >= blueprint.ore_cost:
                build_ore = replace(
                    next_state,
                    ore=(next_state.ore-blueprint.ore_cost),
                    robotOre=(next_state.robotOre + 1)
                )
                next_dp[build_ore] = max(next_dp.get(build_ore, -1), next_geodes)
            if state.robotClay < max_clay and state.ore >= blueprint.clay_cost:
                build_clay = replace(
                    next_state,
                    ore=(next_state.ore - blueprint.clay_cost),
                    robotClay=(next_state.robotClay + 1),
                )
                next_dp[build_clay] = max(next_dp.get(build_clay, -1), next_geodes)
            if state.robotObsidian < max_obsidian and state.ore >= blueprint.obsidian_cost[0] and state.clay >= blueprint.obsidian_cost[1]:
                build_obsidian = replace(next_state,
                    ore=(next_state.ore - blueprint.obsidian_cost[0]),
                    clay=(next_state.clay - blueprint.obsidian_cost[1]),
                    robotObsidian=(next_state.robotObsidian + 1)
                )
                next_dp[build_obsidian] = max(next_dp.get(build_obsidian, -1), next_geodes)
            if state.ore >= blueprint.geode_cost[0] and state.obsidian >= blueprint.geode_cost[1]:
                build_geode = replace(next_state,
                    ore =(next_state.ore - blueprint.geode_cost[0]),
                    obsidian=(next_state.obsidian - blueprint.geode_cost[1]),
                    robotGeode=(next_state.robotGeode + 1)
                )
                next_dp[build_geode] = max(next_dp.get(build_geode, -1), next_geodes)
        dp = next_dp
    print("")
    return best

def solve_blueprint2(blueprint, total_time):
    #dfs + prune
    best = 0

    max_ore = max(blueprint.clay_cost, blueprint.ore_cost, blueprint.obsidian_cost[0], blueprint.geode_cost[0])
    max_clay = blueprint.obsidian_cost[1]
    max_obsidian = blueprint.geode_cost[1]

    cache = set()
    def dfs(remaining_time, state: DpState):
        nonlocal best
        if remaining_time == 0:
            return
            
        if (state, remaining_time) in cache:
           return
        cache.add((state, remaining_time))

        max_spend_ore = (remaining_time-1)*(max_ore)
        max_spend_clay = (remaining_time-1)*(max_clay)
        max_spend_obsidian = (remaining_time-1)*(max_obsidian)

        next_state = replace(
            state,
            ore=min(state.ore + state.robotOre, max_spend_ore),
            clay=min(state.clay + state.robotClay, max_spend_clay),
            obsidian=min(state.obsidian + state.robotObsidian, max_spend_obsidian),
            geodes=(state.geodes + state.robotGeode)
        )
        
        best = max(next_state.geodes, best)

        best_possible = next_state.geodes + remaining_time*next_state.robotGeode + (remaining_time*(remaining_time+1))//2
        
        if best_possible < best:
            return
        
        if state.robotOre < max_ore and state.ore >= blueprint.ore_cost:
            build_ore = replace(
                next_state,
                ore=(next_state.ore-blueprint.ore_cost),
                robotOre=(next_state.robotOre + 1)
            )
            dfs(remaining_time-1, build_ore)

        if state.robotClay < max_clay and state.ore >= blueprint.clay_cost:
            build_clay = replace(
                next_state,
                ore=(next_state.ore - blueprint.clay_cost),
                robotClay=(next_state.robotClay + 1),
            )
            dfs(remaining_time-1, build_clay)
        
        if state.robotObsidian < max_obsidian and state.ore >= blueprint.obsidian_cost[0] and state.clay >= blueprint.obsidian_cost[1]:
            build_obsidian = replace(next_state,
                ore=(next_state.ore - blueprint.obsidian_cost[0]),
                clay=(next_state.clay - blueprint.obsidian_cost[1]),
                robotObsidian=(next_state.robotObsidian + 1)
            )
            dfs(remaining_time-1, build_obsidian)
        


        if state.ore >= blueprint.geode_cost[0] and state.obsidian >= blueprint.geode_cost[1]:
            build_geode = replace(next_state,
                ore =(next_state.ore - blueprint.geode_cost[0]),
                obsidian=(next_state.obsidian - blueprint.geode_cost[1]),
                robotGeode=(next_state.robotGeode + 1)
            )
            dfs(remaining_time-1, build_geode)
        
        dfs(remaining_time-1, next_state)

    starting = DpState(0,0,0,1,0,0,0,0)
    dfs(total_time, starting)
    print(blueprint.id)
    return best

def solve_blueprint3(blueprint, total_time):
    max_ore = max(blueprint.clay_cost, blueprint.ore_cost, blueprint.obsidian_cost[0], blueprint.geode_cost[0])
    max_clay = blueprint.obsidian_cost[1]
    max_obsidian = blueprint.geode_cost[1]
    best = 0

    def dfs(time: int, state: BuildState, bot: int):
        nonlocal best
        best = max(state.geodes, best)

        if bot == 0 and state.robotOre >= max_ore or \
            bot == 1 and state.robotClay >= max_clay or \
            bot == 2 and (state.robotClay == 0 or state.robotObsidian >= max_obsidian) or \
            bot == 3 and (state.robotObsidian == 0):
            return

        best_possible = state.geodes + time*state.robotGeode + (time*(time-1))//2
        if best_possible < best:
            return

        while time > 0:
            if bot == 0 and state.robotOre < max_ore and state.ore >= blueprint.ore_cost:
                build_ore = state.advance(oreCost=blueprint.ore_cost, robotOreDelta=1)
                for i in range(4):
                    dfs(time-1, build_ore, i)
                return
            elif bot == 1 and state.robotClay < max_clay and state.ore >= blueprint.clay_cost:
                build_clay = state.advance(oreCost=blueprint.clay_cost, robotClayDelta=1)
                for i in range(4):
                    dfs(time-1, build_clay, i)
                return
            
            elif bot == 2 and state.robotObsidian < max_obsidian and \
                    state.ore >= blueprint.obsidian_cost[0] and \
                    state.clay >= blueprint.obsidian_cost[1]:
                build_obsidian = state.advance(
                    oreCost=blueprint.obsidian_cost[0],
                    clayCost=blueprint.obsidian_cost[1],
                    robotObsidianDelta=1
                )
                for i in range(4):
                    dfs(time-1, build_obsidian, i)
                return
        
            elif bot == 3 and state.ore >= blueprint.geode_cost[0] and \
                    state.obsidian >= blueprint.geode_cost[1]:
                build_geode = state.advance(
                    oreCost=blueprint.geode_cost[0],
                    obsidianCost=blueprint.geode_cost[1],
                    robotGeodeDelta=1
                )
                for i in range(4):
                    dfs(time-1, build_geode, i)
                return
            state = state.advance()
            best = max(state.geodes, best)
            time -= 1
             

    starting_state = BuildState(0,0,0,1,0,0,0,0)
    dfs(total_time, starting_state, 0)
    dfs(total_time, starting_state, 1)
    return best

def main():
    parser = argparse.ArgumentParser("Day 19")
    parser.add_argument("--solution", type=int, default=0)
    args = parser.parse_args()
    solve_func = solve_blueprint

    if args.solution == 0:
        solve_func = solve_blueprint
    elif args.solution == 1:
        solve_func = solve_blueprint2
    elif args.solution == 2:
        solve_func = solve_blueprint3

    blueprints = [ decode_blueprint(line)  for line in sys.stdin.read().split("\n") ]
    scores = [solve_func(b,24)*b.id for b in blueprints]
    print(scores)
    print(sum(scores))
    scores2 = [solve_func(b,32) for b in blueprints[:3]]
    print(scores2)
    print(reduce(lambda a, b: a*b, scores2))

if __name__ == "__main__":
    main()
