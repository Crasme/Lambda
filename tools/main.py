from mod.Colorprint import colorprint
import os

class Log:
    INFO    = ("green", "INFO")
    WARNING = ("yellow", "WARNING")
    ERROR   = ("red", "ERROR")

log = lambda text, level : colorprint(f"[{level[1]}] {text}", color=level[0])
command = lambda command : os.system(command)


def build():
    log("Lancement du build", Log.INFO)

    command("cd lambda && cargo build")
    command("rm -Rf ./output/*")
    command("mv ./lambda/target/debug/lambda ./output/lambda")

    log("Fin du build", Log.INFO)

def run():
    log("Lancement de lambda", Log.INFO)

    command("cd output && ./lambda")

    log("Arret de lambda", Log.INFO)

import sys
match sys.argv[1]:
    case "build":
        build()
    case "run":
        run()
    case _:
        log("Commande inconnue", Log.ERROR)
        exit(1)