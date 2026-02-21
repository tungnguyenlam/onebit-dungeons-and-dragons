#!/usr/bin/env python3
import subprocess
import os
import sys
import argparse
import time

def run_command(keys, reset=False):
    if reset:
        if os.path.exists("save.toml"):
            os.remove("save.toml")
    
    cmd = ["cargo", "run", "--quiet", "--", "--text", "--step", "-k", keys]
    result = subprocess.run(cmd, capture_output=True, text=True)
    if result.returncode != 0:
        print(f"Error running game: {result.stderr}")
        return None
    return result.stdout

import json

def load_scenarios(path):
    if os.path.exists(path):
        with open(path, "r") as f:
            return json.load(f).get("scenarios", {})
    return {}

def main():
    parser = argparse.ArgumentParser(description="OneBit D&D Visual Check Tool")
    parser.add_argument("keys", nargs="?", help="Sequence of keys to input")
    parser.add_argument("--scenario", "-p", help="Name of a predefined scenario")
    parser.add_argument("--name", "-n", help="Name of the snapshot")
    parser.add_argument("--reset", "-r", action="store_true", help="Reset save state before running")
    parser.add_argument("--show", "-s", action="store_true", help="Print output to stdout")
    parser.add_argument("--dir", default="test_outputs", help="Directory to store outputs")
    parser.add_argument("--config", default="tests/visual_scenarios.json", help="Path to scenarios JSON")
    parser.add_argument("--list-scenarios", "-l", action="store_true", help="List available scenarios")
    parser.add_argument("--verbose-steps", "-v", action="store_true", help="Run each key in the sequence as a separate step and capture/print all outputs")

    args = parser.parse_args()

    scenarios = load_scenarios(args.config)
    
    if args.list_scenarios:
        print("Available Scenarios:")
        for name, data in scenarios.items():
            desc = data.get("description", "No description")
            print(f"  {name:15} : {desc}")
        sys.exit(0)
    
    keys = args.keys
    reset = args.reset
    scenario_name = args.name

    if args.scenario:
        if args.scenario in scenarios:
            s = scenarios[args.scenario]
            keys = s.get("keys", "")
            reset = reset or s.get("reset", False)
            scenario_name = scenario_name or args.scenario
        else:
            print(f"Scenario '{args.scenario}' not found in {args.config}")
            sys.exit(1)

    if not keys and not args.verbose_steps: # If verbose_steps is true, empty keys is allowed for initial state
        print("Error: No keys or scenario provided.")
        parser.print_help()
        sys.exit(1)

    if not os.path.exists(args.dir):
        os.makedirs(args.dir)

    full_output = ""
    if args.verbose_steps:
        # Initial state
        out = run_command("", reset)
        if out:
            full_output += "--- INITIAL STATE ---\n" + out + "\n"
        
        for i, char in enumerate(keys):
            out = run_command(char, False)
            if out:
                full_output += f"--- STEP {i+1}: '{char}' ---\n" + out + "\n"
    else:
        full_output = run_command(keys, reset)
    
    if full_output:
        name = scenario_name if scenario_name else f"check_{int(time.time())}"
        filepath = os.path.join(args.dir, f"{name}.txt")
        
        with open(filepath, "w") as f:
            f.write(full_output)
        
        print(f"Snapshot saved to: {filepath}")
        
        if args.show:
            print(f"\n--- Visual Output: {name} ---")
            print(full_output)
            print("----------------------")

if __name__ == "__main__":
    main()
