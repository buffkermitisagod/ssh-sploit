import os

print("[*] checking root... ", end="")
if os.getuid() != 0:
    print("[FAIL]\n[!] not root! you will have to input code your self!")
    print("[OK]")
    print("""
    welcome to the ssh-sploit installer!

    [1] compile and run 
    [2] run the pre-compiled version

    """)
    i = input("enter option: ")
    print("[*] removing ssh-sploit from /bin/ if there... ")
    print("run: rm /bin/ssh-sploit")

    if i == "1":
        print("[*] compiling...")
        print("run: cargo build")
        print("[*] moving ssh-sploit too bin... ")
        print("run: mv target/debug/ssh-sploit /bin/")
    elif i == "2":
        print("[*] moving ssh-sploit too bin... ")
        print("run: mv comp/ssh-sploit /bin/")
    else:
        print("[!] not an option!")
else:
    print("[OK]")
    print("""
    welcome to the ssh-sploit installer!

    [1] compile and run 
    [2] run the pre-compiled version

    """)
    i = input("enter option: ")
    print("[*] removing ssh-sploit from /bin/ if there... ", end="")
    os.system("rm /bin/ssh-sploit")
    print("[OK]")
    if i == "1":
        print("[*] compiling...")
        os.system("cargo build")
        print("[*] moving ssh-sploit too bin... ", end="")
        os.system("mv target/debug/ssh-sploit /bin/")
        print("[OK]")
    elif i == "2":
        print("[*] moving ssh-sploit too bin... ", end="")
        os.system("mv comp/ssh-sploit /bin/")
        print("[OK]")
    else:
        print("[!] not an option!")

print("[DONE] type 'ssh-sploit' to run ssh-sploit")