from invoke import Collection, task

import setup
from utils.command_executor import CommandExecutor
from utils.windows.environment_utils import activate_VS2022_environment
from utils.logger import print_task_documentation


@task(pre=[setup.setup_context])
@print_task_documentation
def check(ctx):
    """
    Check leet_code rust-lib
    """

    CommandExecutor(ctx)\
        .add_cwd(f"{ctx.leet_code_dir}")\
        .add_command(["cargo check"])\
        .execute("leet_code.check.log")


@task(pre=[setup.setup_context])
@print_task_documentation
def build(ctx):
    """
    Build leet_code rust-lib
    """

    CommandExecutor(ctx)\
        .add_cwd(f"{ctx.leet_code_dir}")\
        .add_command(activate_VS2022_environment())\
        .add_command(["cargo build"])\
        .execute("leet_code.build.log")


@task(pre=[setup.setup_context])
@print_task_documentation
def test(ctx):
    """
    Test leet_code rust-lib
    """

    CommandExecutor(ctx)\
        .add_cwd(f"{ctx.leet_code_dir}")\
        .add_command(activate_VS2022_environment())\
        .add_command(["cargo test"])\
        .execute("leet_code.test.log")


@task(pre=[setup.setup_context])
@print_task_documentation
def full_check(ctx):
    """
    Full check leet_code rust-lib
    """
    check(ctx)
    build(ctx)
    test(ctx)


collection = Collection("leet_code")
collection.add_task(check, name="check")
collection.add_task(build, name="build")
collection.add_task(test, name="test")
collection.add_task(full_check, name="full-check")

import leet_code.macro
collection.add_collection(leet_code.macro)
