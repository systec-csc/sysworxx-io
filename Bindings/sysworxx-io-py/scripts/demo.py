#!/usr/bin/env python3
# SPDX-License-Identifier: LGPL-3.0-or-later
# SPDX-FileCopyrightText: 2025 SYS TEC electronic AG <https://www.systec-electronic.com/>

from time import sleep

import sysworxx_io_py as io

menu_actions = {}


def menu_entry(key):
    def decorator(func):
        menu_actions[key] = func
        return func

    return decorator


def menu_print():
    print("Commands:")
    for c, fn in menu_actions.items():
        print(f"  {c:3} - {fn.__doc__}")


@menu_entry("i")
def show_all_inputs(channels: io.ChannelInfo):
    "dump all inputs"

    print()

    if channels.run_switch:
        print("RUN SWITCH: ", io.get_run_switch())

    if channels.config_switch:
        print("CONFIG SWITCH: ", io.get_config_switch())

    print("\nDigital inputs:")
    for i, name in channels.inputs.items():
        print(name, io.input_get(i))

    print("\nAnalog Inputs (voltage):")
    for i, name in channels.analog_inputs.items():
        io.analog_mode_set(i, io.AnalogMode.VOLTAGE)
        print(name, io.analog_input_get(i))

    print("\nTemperature inputs:")
    for i, name in channels.temp_sensors.items():
        print(name, io.tmp_input_get(i))

    print()


@menu_entry("di")
def digital_input_events(channels: io.ChannelInfo):
    "Use interrupt functionality to wait for digital input events (Stop with CTRL-C)"

    if len(channels.inputs) == 0:
        print("No digital inputs available")

    def input_handler(channel: int, state: bool):
        print(f"Channel {channel} = {state}")

    registered = set()

    for i, c in channels.inputs.items():
        try:
            io.input_register_callback(i, input_handler)
            print(f"Registered handler for {c} ({i})")
            registered.add(i)
        except:
            # Not all inputs allow event based input handling
            continue

    try:
        while True:
            sleep(5.0)
    except KeyboardInterrupt:
        pass

    for i, c in channels.inputs.items():
        if i not in registered:
            continue
        print(f"Un-register handler for {c} ({i})")
        io.input_unregister_callback(i, input_handler)


@menu_entry("do")
def toggle_digital_outputs(channels: io.ChannelInfo):
    "toggle all digital outputs"

    if len(channels.outputs) == 0:
        print("No output channels available")

    print("\nEnable all outputs")

    if channels.run_led:
        print("RUN LED=True")
        io.set_run_led(True)
    if channels.err_led:
        print("ERR LED=True")
        io.set_err_led(True)

    for i, name in channels.outputs.items():
        io.output_set(i, True)
        print(f"{name}=True")
        sleep(0.1)

    sleep(1.0)

    print("\nDisable all outputs")

    if channels.run_led:
        print("RUN LED=False")
        io.set_run_led(False)
    if channels.err_led:
        print("ERR LED=False")
        io.set_err_led(False)

    for i, name in channels.outputs.items():
        io.output_set(i, False)
        print(f"{name}=False")
        sleep(0.1)


@menu_entry("cnt")
def toggle_digital_outputs(channels: io.ChannelInfo):
    "Enable counter inputs and print value for 10s"

    if len(channels.counter_inputs) == 0:
        print("No counter channels available")

    for i, c in channels.counter_inputs.items():
        io.cnt_enable(i, False)
        io.cnt_set_preload(i, 0)
        io.cnt_setup(i, io.CntMode.COUNTER, io.CntTrigger.ANYEDGE, io.CntDirection.UP)
        io.cnt_enable(i, True)

    for i in range(20):
        sleep(0.5)
        for i, c in channels.counter_inputs.items():
            print(f"{c}={io.cnt_get(i)} ", end="")
        print()

    for i, c in channels.counter_inputs.items():
        io.cnt_enable(i, False)


@menu_entry("ab")
def ab_encoder(channels: io.ChannelInfo):
    "Setup and enable counter inputs in A/B encoder mode and print value for 10s"

    if len(channels.counter_inputs) == 0:
        print("No counter channels available")

    for i, c in channels.counter_inputs.items():
        io.cnt_enable(i, False)
        io.cnt_set_preload(i, 0)
        io.cnt_setup(i, io.CntMode.ABENCODER, io.CntTrigger.ANYEDGE, io.CntDirection.UP)
        io.cnt_enable(i, True)

    for i in range(20):
        sleep(0.5)
        for i, c in channels.counter_inputs.items():
            print(f"{c}={io.cnt_get(i)} ", end="")
        print()

    for i, c in channels.counter_inputs.items():
        io.cnt_enable(i, False)


@menu_entry("pwm")
def pwm_demo(channels: io.ChannelInfo):
    "Enable PWM output signal on each channel for 5s"

    if len(channels.pwm_outputs) == 0:
        print("No PWM output channels available")

    for i, c in channels.pwm_outputs.items():
        print(f"Setup {c} ({i})")
        io.pwm_set_timebase(i, io.PwmTimebase.MS1)
        io.pwm_setup(i, 400, 100 * (i + 1))
        print(f"Enable {c} ({i})")
        io.pwm_enable(i, True)

    sleep(5.0)

    for i, c in channels.pwm_outputs.items():
        print(f"Disable {c} ({i})")
        io.pwm_enable(i, False)


@menu_entry("ao")
def analog_output_set(_: io.ChannelInfo):
    """Set analog outputs"""
    raise NotImplementedError
    # FIXME: test this with a device with analog outputs
    # for i, c in channels.analog_outputs.items():
    #     value = 1.0 * (i + 1)
    #     print(f"Set DO {c} ({i}) to {value}")
    #     io.analog_output_set(i, value)


@menu_entry("q")
def quit_app(_: io.ChannelInfo):
    "quit application"
    raise KeyboardInterrupt


def main():
    print()
    print("sysworxx-io-py Demo Application")
    print()

    channels = io.get_channel_info()
    __import__("pprint").pprint(channels)

    try:
        while True:
            menu_print()
            input_selection = input("Enter command: ").lower()
            menu_action = menu_actions.get(input_selection)
            if menu_action:
                menu_action(channels)
    except (KeyboardInterrupt, EOFError):
        print("\nStopping application")


if __name__ == "__main__":
    main()
