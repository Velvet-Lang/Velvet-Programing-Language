# vel/init.py
import os

def init_project():
    os.makedirs("src", exist_ok=True)
    with open("src/main.vel", "w") as f:
        f.write('# Velvet project initialized\n')
    print("Project initialized with src/main.vel")
