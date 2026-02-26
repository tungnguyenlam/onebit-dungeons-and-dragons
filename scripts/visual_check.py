#!/usr/bin/env python3
import subprocess
import os
import sys
import argparse
from datetime import datetime

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


def validate_expectations(output, scenario_data):
    missing = []
    for token in scenario_data.get("expected_contains", []):
        if token not in output:
            missing.append(token)

    forbidden = []
    for token in scenario_data.get("expected_not_contains", []):
        if token in output:
            forbidden.append(token)

    return missing, forbidden

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
    parser.add_argument(
        "--artifact",
        choices=["final", "full", "none"],
        default="final",
        help="Artifact mode: final=save only final state (default), full=save every step, none=do not write a file",
    )
    parser.add_argument(
        "--history",
        action="store_true",
        help="Keep timestamped history files instead of overwriting the latest snapshot name",
    )

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
    scenario_data = None

    if args.scenario:
        if args.scenario in scenarios:
            s = scenarios[args.scenario]
            scenario_data = s
            scenario_keys = s.get("keys", "")
            keys = scenario_keys + (keys if keys else "")
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
    final_output = ""
    if args.verbose_steps:
        # Capture initial state once so verbose runs can include a clean baseline.
        out = run_command("", reset)
        if out:
            full_output += "--- INITIAL STATE ---\n" + out + "\n"
            final_output = out
        
        for i, char in enumerate(keys):
            out = run_command(char, False)
            if out:
                full_output += f"--- STEP {i+1}: '{char}' ---\n" + out + "\n"
                final_output = out
    else:
        full_output = run_command(keys, reset)
        final_output = full_output
    
    if full_output:
        if scenario_data:
            missing, forbidden = validate_expectations(final_output, scenario_data)
            if missing or forbidden:
                print(f"Scenario expectation failed: {args.scenario}")
                for token in missing:
                    print(f"  missing: {token}")
                for token in forbidden:
                    print(f"  unexpected: {token}")
                sys.exit(2)

        name = scenario_name if scenario_name else "latest"
        if args.history:
            stamp = datetime.now().strftime("%Y%m%d-%H%M%S")
            name = f"{name}_{stamp}"
        filepath = os.path.join(args.dir, f"{name}.txt")

        if args.artifact != "none":
            to_write = full_output if args.artifact == "full" else final_output
            with open(filepath, "w") as f:
                f.write(to_write)
            print(f"Snapshot saved to: {filepath}")
        
        if args.show:
            print(f"\n--- Visual Output: {name} ---")
            if args.artifact == "full":
                print(full_output)
            else:
                print(final_output)
            print("----------------------")

if __name__ == "__main__":
    main()
