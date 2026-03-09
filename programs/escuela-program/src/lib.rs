use anchor_lang::prelude::*;

declare_id!("GPk2ZRNvV6QrAnn9CUJyeJ1w3DdY1Gr9ogyeCnspzsrm");

#[program]
pub mod escuela_program {
    use super::*;

    pub fn crear_escuela(ctx: Context<CrearEscuela>, nombre: String) -> Result<()> {
        let escuela = &mut ctx.accounts.escuela;
        escuela.nombre = nombre;
        escuela.owner = *ctx.accounts.owner.key;
        escuela.alumnos = Vec::new();
        Ok(())
    }

    pub fn agregar_alumno(
        ctx: Context<AgregarAlumno>,
        nombre: String,
        matricula: String,
        edad: u8,
        activo: bool,
    ) -> Result<()> {
        let escuela = &mut ctx.accounts.escuela;

        if escuela.alumnos.len() >= 100 {
            return Err(ErrorCode::MaxAlumnosReached.into());
        }

        let nuevo_alumno = Alumno {
            nombre,
            matricula,
            edad,
            activo,
        };

        escuela.alumnos.push(nuevo_alumno);

        msg!("Alumno agregado exitosamente");

        Ok(())
    }

    pub fn eliminar_alumno(ctx: Context<EliminarAlumno>, matricula: String) -> Result<()> {
        let escuela = &mut ctx.accounts.escuela;

        if let Some(pos) = escuela.alumnos.iter().position(|a| a.matricula == matricula) {
            escuela.alumnos.remove(pos);
            msg!("Alumno con matrícula '{}' eliminado", matricula);
            Ok(())
        } else {
            err!(ErrorCode::AlumnoNoEncontrado)
        }
    }

    pub fn editar_alumno(
        ctx: Context<EditarAlumno>,
        matricula: String,
        nuevo_nombre: Option<String>,
        nuevo_edad: Option<u8>,
        nuevo_activo: Option<bool>,
    ) -> Result<()> {
        let escuela = &mut ctx.accounts.escuela;

        if let Some(alumno) = escuela.alumnos.iter_mut().find(|a| a.matricula == matricula) {
            if let Some(nombre) = nuevo_nombre {
                alumno.nombre = nombre;
            }
            if let Some(edad) = nuevo_edad {
                alumno.edad = edad;
            }
            if let Some(activo) = nuevo_activo {
                alumno.activo = activo;
            }
            msg!("Alumno con matrícula '{}' editado correctamente", matricula);
            Ok(())
        } else {
            err!(ErrorCode::AlumnoNoEncontrado)
        }
    }
}

#[account]
#[derive(InitSpace)]
pub struct Escuela {
    #[max_len(100)]
    pub nombre: String,

    pub owner: Pubkey,

    #[max_len(100)]
    pub alumnos: Vec<Alumno>,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, InitSpace)]
pub struct Alumno {
    #[max_len(60)]
    pub nombre: String,

    #[max_len(20)]
    pub matricula: String,

    pub edad: u8,

    pub activo: bool,
}

#[derive(Accounts)]
pub struct CrearEscuela<'info> {
    #[account(
        init,
        payer = owner,
        space = 8 + Escuela::INIT_SPACE,
        seeds = [b"escuela", owner.key().as_ref()],
        bump
    )]
    pub escuela: Account<'info, Escuela>,

    #[account(mut)]
    pub owner: Signer<'info>,

    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct AgregarAlumno<'info> {
    #[account(
        mut,
        seeds = [b"escuela", owner.key().as_ref()],
        bump,
        has_one = owner
    )]
    pub escuela: Account<'info, Escuela>,

    #[account(mut)]
    pub owner: Signer<'info>,
}

#[derive(Accounts)]
pub struct EliminarAlumno<'info> {
    #[account(
        mut,
        seeds = [b"escuela", owner.key().as_ref()],
        bump,
        has_one = owner
    )]
    pub escuela: Account<'info, Escuela>,

    #[account(mut)]
    pub owner: Signer<'info>,
}

#[derive(Accounts)]
pub struct EditarAlumno<'info> {
    #[account(
        mut,
        seeds = [b"escuela", owner.key().as_ref()],
        bump,
        has_one = owner
    )]
    pub escuela: Account<'info, Escuela>,

    #[account(mut)]
    pub owner: Signer<'info>,
}

#[error_code]
pub enum ErrorCode {
    #[msg("Se alcanzó el número máximo de alumnos (100).")]
    MaxAlumnosReached,

    #[msg("Alumno no encontrado.")]
    AlumnoNoEncontrado,
}