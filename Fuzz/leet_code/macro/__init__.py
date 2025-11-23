from invoke import Collection, task

import setup
from utils.command_executor import CommandExecutor
from utils.windows.environment_utils import activate_VS2022_environment
from utils.logger import print_task_documentation


@task(pre=[setup.setup_context])
@print_task_documentation
def check(ctx):
    """
    Check leet_code_macro rust-lib
    """

    CommandExecutor(ctx)\
        .add_cwd(f"{ctx.leet_code_macro_dir}")\
        .add_command(["cargo check"])\
        .execute("leet_code.macro.check.log")


@task(pre=[setup.setup_context])
@print_task_documentation
def build(ctx):
    """
    Build leet_code_macro rust-lib
    """

    CommandExecutor(ctx)\
        .add_cwd(f"{ctx.leet_code_macro_dir}")\
        .add_command(activate_VS2022_environment())\
        .add_command(["cargo build"])\
        .execute("leet_code.macro.build.log")


collection = Collection("macro")
collection.add_task(check, name="check")
collection.add_task(build, name="build")
