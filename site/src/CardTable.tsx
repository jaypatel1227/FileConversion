import React from 'react';

export interface ICardProps {
  name: string;
  description: string;
  serviceFucntionName: string;
}

const Card: React.FC<ICardProps> = (props) => {
  let { name, description, serviceFucntionName } = props;

  return (
    <div>
      <div className='cardTitle'> {name} </div>
      <div className='cardDesciption'> {description} </div>
    </div>
  );
};

interface ICardRowProps {
  cards: ICardProps[];
  initialIndex: number;
}

const CardRow: React.FC<ICardRowProps> = (props) => {

  return (
    <>
      {props.cards.map((card, index) => (
        <td key={index + props.initialIndex} className='cardItem'>
          <Card name={card.name} description={card.description} serviceFucntionName={card.serviceFucntionName} />
        </td>
      ))}
    </>
  );
}

export interface ICardTableProps {
  cards?: ICardProps[];
}

export const CardTable = (props: ICardTableProps) => {

  let NUM_COLS = 3;

  const groupedCards = props.cards?.reduce<ICardProps[][]>((arr, card, index) => {
    if (index % NUM_COLS === 0) arr.push([]);
    arr[arr.length - 1].push(card);
    return arr;
  }, []);

  return (
    <table>
      <tbody>
        {groupedCards?.map((group, index) => (
          <tr>
            <CardRow cards={group} initialIndex={index * NUM_COLS} />
          </tr>
        ))}
      </tbody>
    </table>
  );
};

export default CardTable;
