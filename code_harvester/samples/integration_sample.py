class Animal:
    def __init__(self, name, species):
        self.name = name
        self.species = species

    def speak(self):
        print(f"My name is {self.name} and I am a {self.species}")

class Dog(Animal):
    def __init__(self, name, breed):
        super().__init__(name, "Dog")
        self.breed = breed

    def speak(self):
        super().speak()
        print(f"I am a {self.breed}")

def add(a, b):
    return a + b

def multiply(a, b):
    return a * b

class Calculator:
    @staticmethod
    def add(a, b):
        return a + b

    @staticmethod
    def multiply(a, b):
        return a * b

    @classmethod
    def subtract(cls, a, b):
        return a - b

    @classmethod
    def divide(cls, a, b):
        if b == 0:
            raise ValueError("Division by zero is not allowed")
        return a / b
