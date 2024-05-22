import React from 'react';

export interface ICardProps {
  name: string;
  description: string;
  fileExtension: string;
  conversionCallback: () => void;
}

const Card: React.FC<ICardProps> = (props) => {
  let { name, description, fileExtension, conversionCallback } = props;

  return (
    <div className='_container'>
      <div className='_cardTitle'> {name} </div>
      <div className='_cardDescription'> {description} </div>
      <ConvertRequestButton fileExtension={fileExtension} conversionCallback={conversionCallback} />
    </div>
  );
};

interface IConvertRequestButtonProps {
  fileExtension: string;
  conversionCallback: () => void;
}

const ConvertRequestButton: React.FC<IConvertRequestButtonProps> = (props) => {
  const conversionOnChange: React.ChangeEventHandler = (e: React.ChangeEvent) => {
    if (e.isTrusted && !!e.target) {
      return e.target; //TODO: Handle the button
    }
  };

  return (
    <div>
      <input className='_convertButton' type='file' accept={props.fileExtension} onChange={conversionOnChange} />
      <button type='submit' onClick={props.conversionCallback} >Send Convert Request</button>
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
        <td key={index + props.initialIndex} className='_cardItem'>
          <Card name={card.name} description={card.description} fileExtension={card.fileExtension} conversionCallback={card.conversionCallback} />
        </td>
      ))}
    </>
  );
};

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
    <table className='_serviceGrid'>
      <tbody>
        {groupedCards?.map((group, index) => (
          <tr className='_tableRow'>
            <CardRow cards={group} initialIndex={index * NUM_COLS} />
          </tr>
        ))}
      </tbody>
    </table>
  );
};

export default CardTable;
