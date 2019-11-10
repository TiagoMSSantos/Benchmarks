type TestClass = ref object of RootObj
  name: string
  age: int
method vocalize(this: TestClass): string {.base.} = "..."
method ageHumanYrs(this: TestClass): int {.base.} = this.age

type Dog = ref object of TestClass
method vocalize(this: Dog): string = "woof"
method ageHumanYrs(this: Dog): int = this.age * 7

type Cat = ref object of TestClass
method vocalize(this: Cat): string = "meow"


var TestClasses: seq[TestClass] = @[]
TestClasses.add(Dog(name: "Sparky", age: 10))
TestClasses.add(Cat(name: "Mitten", age: 10))

for a in TestClasses:
  echo a.vocalize()
  echo a.ageHumanYrs()