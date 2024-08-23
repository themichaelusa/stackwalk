import os
import shutil
import subprocess

if __name__ == "__main__":
    if not os.path.exists("stackwalk"):
        clone_cmd = "git clone https://github.com/getAsterisk/stackwalk.git"
        subprocess.run(clone_cmd, shell=True, cwd="./")
        # # run the get_grammar.sh script
        subprocess.run("./get_grammar.sh", shell=True, cwd="./stackwalk")
    # build the stackwalk crate
    subprocess.run("cargo build", shell=True, cwd="./stackwalk")