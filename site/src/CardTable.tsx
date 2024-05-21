import React from 'react';

export interface ICardProps {
        name: string;
        description: string;
        serviceFucntionName: string;
        fileExtension: string;
        // converisonCallback: () => void;
}

const Card: React.FC<ICardProps> = (props) => {
        let { name, description, serviceFucntionName, fileExtension } = props;

        const conversionCallback: React.ChangeEventHandler = (e: React.ChangeEvent) => {
                if (!!e.target.value && e.isTrusted) {
                        const resp = await fetch('http://localhost:5001/available_options', { method: "GET", });
                }
                return;
        };

        return (
                <div className='_container'>
                        <div className='_cardTitle'> {name} </div>
                        <div className='_cardDescription'> {description} </div>
                        <input className='_convertButton' type='file' accept={fileExtension} onChange={conversionCallback} />
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
                                        <Card name={card.name} description={card.description} serviceFucntionName={card.serviceFucntionName} fileExtension={""} />
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
