const data = [
  [
    {
      name: "nickname",
      value: "Fijisoo"
    },
    {
      name: "current vibe",
      value: "healing..."
    }
  ],
  [
    {
      name: "favourite device",
      value: "iPhone 14"
    },
    {
      name: "operating system",
      value: "macOS"
    },
    {
      name: "recent purchase",
      value: "Noise-canceling headphones"
    }
  ],
  [
    {
      name: "hobbies",
      value: "Photography, Hiking"
    }
  ],
  [
    {
      name: "current game",
      value: "Apex Legends"
    },
    {
      name: "gamer tag",
      value: "AJonFire"
    },
    {
      name: "platform",
      value: "PC, PlayStation"
    }
  ],
  [
    {
      name: "favourite song",
      value: "Fred again... - Marea"
    },
    {
      name: "favorite game",
      value: "The Legend of Zelda: Breath of the Wild"
    },
    {
      name: "favorite book",
      value: "Harry Potter and the Sorcerer's Stone"
    },
    {
      name: "favorite place",
      value: "Japan"
    },
    {
      name: "favorite workout",
      value: "Running"
    }
  ],
  [
    {
      name: "short-term goal",
      value: "Learn React.js"
    },
    {
      name: "long-term goal",
      value: "Start my own tech company"
    },
    {
      name: "life motto",
      value: "Stay curious, stay humble."
    }
  ]
];

export const HeroText = () => {
  return (
    <div className="flex mt-10 flex-col">
      {data.map((value) => {
        return (
          <span className="flex flex-col items-start py-2 opacity-30">
            {value.map((el) => {
              return (
                <div className="flex flex-nowrap">
                  <span className="font-spaceMono text-[12px] text-white">
                    {el.name}:
                  </span>
                  <span className="ml-2 font-sequel100Black text-[12px] font-55 text-white">
                    {el.value}
                  </span>
                </div>
              );
            })}
          </span>
        );
      })}
    </div>
  );
};
